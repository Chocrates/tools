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

const TEN_SECONDS = 10 * 1000;

const getPermissionName = (permissionObj) => {
    // I think these have to be in order since the object returns true for all the values team has
    if (permissionObj["admin"]) {
        return "admin";
    } else if (permissionObj["maintain"]) {
        return "maintain";
    } else if (permissionObj["push"]) {
        return "push";
    } else if (permissionObj["triage"]) {
        return "triage";
    } else if (permissionObj["pull"]) {
        return "pull";
    } else {
        return null;
    }
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

        const repos = JSON.parse(fs.readFileSync(path, "utf8"));
        /*
        [
          {
              "name": "repo",
              "teams": ["team","team"]
          }
        ]
        
         */
        for (let repo of repos) {
            try {
                // Get default branch
                const repoInfo = await client.repos.get({
                    owner: sourceOrg,
                    repo: repo.name,
                });
                console.log(`Cloning repo ${sourceOrg}/${repo.name}`);
                let result = await git.clone(
                    `https://${token}@github.com/${sourceOrg}/${repo.name}.git`
                );

                result = await git.spawn(
                    ["checkout", "-b", "fix-org-references"],
                    {
                        cwd: repo.name,
                    }
                );

                const files = glob.sync(`${repo.name}/**/*`);
                const filesToAlter = files.filter(
                    (file) => file.indexOf(".git") < 0
                );
                for (let file of filesToAlter) {
                    if (!fs.lstatSync(file).isDirectory()) {
                        const fileData = fs.readFileSync(`${file}`, "utf8");
                        var replace = fileData.replace(
                            new RegExp(sourceOrg, "ig"),
                            destOrg
                        );

                        fs.writeFileSync(`${file}`, replace, "utf8");
                    }
                }

                result = await git.spawn(["add", "."], {
                    cwd: repo.name,
                });
                if (result.stdout !== "") {
                    result = await git.spawn(
                        ["commit", "-m", '"Altering org references"'],
                        {
                            cwd: repo.name,
                        }
                    );
                    result = await git.spawn(
                        ["push", "-u", "origin", "fix-org-references"],
                        {
                            cwd: repo.name,
                        }
                    );
                    await client.pulls.create({
                        owner: sourceOrg,
                        repo: repo.name,
                        head: "fix-org-references",
                        base: repoInfo.data.default_branch,
                        title: `Migrate ${sourceOrg} references`,
                    });
                }
                // Get Team Id's
                teams = [];
                for (let team of repo.teams) {
                    teamObj = { name: team };
                    let permissionResp;
                    try {
                        permissionResp = await client.teams.checkPermissionsForRepoInOrg(
                            {
                                org: sourceOrg,
                                team_slug: team,
                                owner: sourceOrg,
                                repo: repo.name,
                                headers: {
                                    accept:
                                        "application/vnd.github.v3.repository+json",
                                },
                            }
                        );
                    } catch (e) {
                        if (e.status === 404) {
                            //Team did not have access to repo originally, so giving it read access
                            permissionResp = {
                                data: {
                                    permissions: {
                                        admin: false,
                                        maintain: false,
                                        push: false,
                                        triage: false,
                                        pull: true,
                                    },
                                },
                            };
                        } else {
                            throw e;
                        }
                    }
                    teamObj.permissions = permissionResp.data.permissions;
                    const dstResp = await client.teams.getByName({
                        org: destOrg,
                        team_slug: team,
                    });
                    teamObj.id = dstResp.data.id;
                    teams.push(teamObj);
                }

                await client.repos.transfer({
                    owner: sourceOrg,
                    repo: repo.name,
                    new_owner: destOrg,
                    team_ids: teams.map((team) => team.id),
                });

                sleep(TEN_SECONDS);
                for (let team of teams) {
                    const resp = await client.teams.addOrUpdateRepoPermissionsInOrg(
                        {
                            org: destOrg,
                            team_slug: team.name,
                            owner: destOrg,
                            repo: repo.name,
                            permission: getPermissionName(team.permissions),
                        }
                    );
                }
            } catch (e) {
                console.error(e);
            } finally {
                fs.rmdirSync(repo.name, { recursive: true });
            }
        }
    } catch (e) {
        console.error(e);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
