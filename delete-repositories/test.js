const { throttling } = require("@octokit/plugin-throttling");
const { retry } = require("@octokit/plugin-retry");
const { Octokit } = require("@octokit/rest");
const MyOctokit = Octokit.plugin(throttling).plugin(retry);
const csv = require("csvtojson");
const { array } = require("yargs");

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
    .option("repo_input", {
        alias: "r",
        description: "Repo on which we are working",
        global: true,
        demandOption: true,
    }).argv;
async function main() {
    const token = argv.token;
    const owner = argv.owner;
    const csvFilePath = argv.repo_input;
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

    const jsonArray = await csv().fromFile(csvFilePath);
    const org = new Set();
    for (let line of jsonArray) {
        org.add(line["Organization"]);
    }
    let migrations = [];
    for (let i = 0; i < Array.from(org).length; i++) {
        const options = client.migrations.listForOrg.endpoint.merge({
            org: Array.from(org)[i],
            per_page: 5,
        });
        const org_migration = await client.paginate(options);
        migrations = migrations.concat(org_migration);
    }
    for (let migration of migrations) {
        for (let repo of migration.repositories) {
            for (let line of jsonArray) {
                if (
                    line["Organization"] === repo.full_name.split("/")[0] &&
                    line["Repo Name"] === repo.full_name.split("/")[1]
                ) {
                    line["migration_id"] = migration.id;
                    line["repo_id"] = repo.node_id;
                    break;
                }
            }
        }
    }
    console.log(JSON.stringify(jsonArray, null, 4));
    for (let line of jsonArray) {
        try {
            let response = await client.migrations.unlockRepoForOrg({
                org: line["Organization"],
                repo_name: line["Repo Name"],
                migration_id: line["migration_id"],
            });
            console.log("something");
        } catch (error) {
            console.error(JSON.stringify(error));
        }

        try {
            //core.info(`Unarchiving repository with ID ${repoID}`)
            await client.graphql(
                `mutation UnArchiveRepository($repoID: String!) {
                    unarchiveRepository(input: {repositoryId: $repoID}) {
                        repository {
                          isArchived
                        }
                  }
                }`,
                {
                    repoID: `${line["repo_id"]}`,
                }
            );
        } catch (error) {
            console.error(JSON.stringify(error));
        }
    }
}
main();
