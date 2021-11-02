const { throttling } = require("@octokit/plugin-throttling");
const { retry } = require("@octokit/plugin-retry");
const { Octokit } = require("@octokit/rest");
const parse = require("csv-parse/lib/sync");
const fs = require("fs");

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
            .option("file", {
                alias: "f",
                description: "CSV File of repos to delete",
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

        // open file
        let fileContents = fs.readFileSync(argv.file).toString();

        let records = parse(fileContents);
        let first = true;
        for (let record of records) {
            if (first) {
                first = false;
                continue;
            }

            // delete repositories
            await client.repos.delete({
                owner: record[0].split("/")[0],
                repo: record[0].split("/")[1],
            });

            console.log("deleted", record);
        }
    } catch (e) {
        console.error(e);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
