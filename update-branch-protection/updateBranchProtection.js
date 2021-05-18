const { throttling } = require("@octokit/plugin-throttling");
const { Octokit } = require("@octokit/rest");
const fs = require("fs");
const input = JSON.parse(fs.readFileSync("properties.json"));
const MyOctokit = Octokit.plugin(throttling);

/**
 * main
 * runs the updateBranchProtection octokit api
 * for the repos specified in properties.json
 */
async function main() {
    const client = new MyOctokit({
        auth: `token ${input.token}`,
        baseUrl: input.githubUrl,
        previews: ["luke-cage"],
        throttle: {
            onRateLimit: (retryAfter, options) => {
                console.warn(`Request quota exhausted for request 
          ${options.method} ${options.url}`);
                console.log(`Retrying after ${retryAfter} seconds! 
          Retry Count: ${options.request.retryCount}`);
                return true;
            },
            onAbuseLimit: (retryAfter, options) => {
                console.warn(`Abuse detected for request 
          ${options.method} ${options.url}`);
            },
        },
    });

    try {
        for (const repo of input.repos) {
            console.log(`Repo: ${repo} ${input.organization} ${input.branch}`);
            const results = await client.repos.updateBranchProtection({
                owner: input.organization,
                repo,
                branch: input.branch,
                required_status_checks: input.required_status_checks,
                enforce_admins: input.enforce_admins,
                required_pull_request_reviews:
                    input.required_pull_request_reviews,
                restrictions: input.restrictions,
                allow_force_pushes: input.allow_force_pushes,
                allow_deletions: input.allow_deletions,
            });
            console.log(JSON.stringify(results));
        }
    } catch (error) {
        console.log(error);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
