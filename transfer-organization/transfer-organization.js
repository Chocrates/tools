const { throttling } = require("@octokit/plugin-throttling");
const { retry } = require("@octokit/plugin-retry");
const { Octokit } = require("@octokit/rest");
const fs = require("fs");
const git = require("@npmcli/git");
const glob = require("glob");

const MyOctokit = Octokit.plugin(throttling).plugin(retry);
const sleep = (ms) => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};

async function main() {
    try {
        const argv = require("yargs")
            .option("token", {
                alias: "t",
                description: "personal access token with which to authenticate",
                global: true,
                demandOption: true,
            })
            .option("source-organization", {
                alias: "s",
                description: "Source organization for the repos",
                global: true,
                demandOption: true,
            })
            .option("destination-organization", {
                alias: "d",
                description: "Destination organization for the repos",
                global: true,
                demandOption: true,
            })
            .option("file", {
                alias: "f",
                description: "path to file with repositories",
                global: true,
                demandOption: true,
            }).argv;

        const client = new MyOctokit({
            auth: `token ${argv.token}`,
            throttle: {
                onRateLimit: (retryAfter, options) => {
                    console.warn(
                        `Request quota exhausted for request ${options.method} ${options.url}`
                    );
                    console.warn(
                        `Retrying after ${retryAfter} seconds! Retry Count: ${options.request.retryCount}`
                    );
                    return true;
                },
                onAbuseLimit: (retryAfter, options) => {
                    console.warn(
                        `Abuse detected for request ${options.method} ${options.url}`
                    );
                    return true;
                },
            },
        });

        let token = argv.token;
        let sourceOrg = argv.sourceOrganization;
        let destOrg = argv.destinationOrganization;
        let path = argv.file;

        // get repos
        const repos = fs.readFileSync(path, "utf8").trim().split("\n");

        for (let repo of repos) {
            // Get default branch
            const repoInfo = await client.repos.get({
                owner: sourceOrg,
                repo: repo,
            });
            console.log(`Cloning repo ${sourceOrg}/${repo}`);
            git.clone(`https://${token}@github.com/${sourceOrg}/${repo}.git`)
                .then(() => {
                    console.log("Checking out new branch");
                    git.spawn(["checkout", "-b", "fix-org-references"], {
                        cwd: repo,
                    });
                })
                .then(() => {
                    console.log("Replacing org instances in repo");
                    const files = glob.sync(`${repo}/**/*`);
                    const filesToAlter = files.filter(
                        (file) => file.indexOf(".git") < 0
                    );
                    for (let file of filesToAlter) {
                        if (!fs.lstatSync(file).isDirectory()) {
                            const fileData = fs.readFileSync(`${file}`, "utf8");
                            var result = fileData.replace(
                                new RegExp(sourceOrg, "g"),
                                destOrg
                            );

                            fs.writeFileSync(`${file}`, result, "utf8");
                        }
                    }
                })
                .then(() => {
                    console.log("Adding changed files");
                    git.spawn(["add", "."]);
                })
                .then(() => {
                    console.log("Comming changes");
                    git.spawn(["commit", "-m", '"Altering org references"']);
                })
                .then(() => {
                    console.log("Pushing branch up to origin");
                    git.spawn(["push", "-u", "origin", "fix-org-references"]);
                })
                .then(() => {
                    // Create PR for new changes
                    client.pulls.create({
                        owner: sourceOrg,
                        repo: repo,
                        head: "fix-org-references",
                        base: repoInfo.data.default_branch,
                    });
                })
                .catch((err) => console.error(err))
                .finally(() => {
                    console.log("Cleaning up the directory");
                    fs.rmdirSync(repo, { recursive: true });
                    console.log("Am I failing here?");
                });
        }

        // const options = client.rest.search.code.endpoint.merge({
        //     q: query,
        //     per_page: 100,
        // });

        // const search_results = await client.paginate(options);
    } catch (e) {
        console.error(e);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
