const { throttling } = require("@octokit/plugin-throttling");
const { retry } = require("@octokit/plugin-retry");
const { Octokit } = require("@octokit/rest");
const MyOctokit = Octokit.plugin(throttling).plugin(retry);

const argv = require("yargs")
    .option("token", {
        alias: "t",
        description: "personal access token with which to authenticate",
        global: true,
        demandOption: true,
    })
    .option("owner", {
        alias: "o",
        description: "Owner or Org of the repo",
        global: true,
        demandOption: true,
    })
    .option("repo", {
        alias: "r",
        description: "Repo on which we are working",
        global: true,
        demandOption: true,
    }).argv;

async function main() {
    const token = argv.token;
    const owner = argv.owner;
    const repo = argv.repo;

    const client = new MyOctokit({
        auth: "token " + token,
        throttle: {
            onRateLimit: (retryAfter, options) => {
                console.warn(
                    `Request quota exhausted for request ${options.method} ${options.url}`
                );
                console.log(
                    `Retrying after ${retryAfter} seconds! Retry Count: ${options.request.retryCount}`
                );
                return true;
            },
            onAbuseLimit: (retryAfter, options) => {
                console.warn(
                    `Abuse detected for request ${options.method} ${options.url}`
                );
            },
        },
        request: {
            retries: 1,
            doNotRetry: [403],
        },
    });

    const todays_date = new Date().toISOString().split("T")[0];

    const label = `due-on: ${todays_date}`;

    let issues = await client.issues.listForRepo({
        owner,
        repo,
        state: "open",
        labels: label,
    });

    console.log(JSON.stringify(issues));
}

main();
