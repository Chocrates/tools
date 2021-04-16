const { throttling } = require("@octokit/plugin-throttling");
const { retry } = require("@octokit/plugin-retry");
const { Octokit } = require("@octokit/rest");

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
            .option("query", {
                alias: "q",
                description: "Search query",
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

        let query = argv.query;
        const options = client.rest.search.code.endpoint.merge({
            q: query,
            per_page: 100,
        });

        const search_results = await client.paginate(options);

        let repos = {};
        for (let result of search_results) {
            const repoOwner = result.repository.owner.login;
            const repoName = result.repository.name;
            const response = await client.repos.listCollaborators({
                owner: repoOwner,
                repo: repoName,
            });

            const collaborators = response["data"].map(
                (collab) => collab.login
            );
            const users = [];
            for (let collaborator of collaborators) {
                const resp = await client.users.getByUsername({
                    username: collaborator,
                });

                users.push({
                    login: collaborator,
                    email: resp["data"].email,
                });
            }

            repos[repoName] = {
                users,
            };
        }

        console.log(JSON.stringify(repos));
    } catch (e) {
        console.error(e);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
