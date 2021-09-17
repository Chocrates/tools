const { throttling } = require("@octokit/plugin-throttling");
const { Octokit } = require("@octokit/rest");
const fs = require("fs");
const input = JSON.parse(fs.readFileSync("properties.json"));
const MyOctokit = Octokit.plugin(throttling);

mapTeamToPermission = (team) => {};

/**
 * main
 */

const getPermission = (permissions) => {
    if (permissions["admin"] === true) {
        return "admin";
    } else if (permissions["triage"] === true) {
        return "triage";
    } else if (permissions["maintain"] === true) {
        return "maintain";
    } else if (permissions["push"] === true) {
        return "push";
    } else {
        return "pull";
    }
};
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
            let options = client.repos.listTeams.endpoint.merge({
                owner: input.organization,
                repo,
            });

            const currentTeams = await client.paginate(options);

            const newTeams = {};
            for (const [permission, team] of Object.entries(input.teams)) {
                // create new teams
                try {
                    newTeams[permission] = await client.teams.create({
                        org: input.organization,
                        name: team.replace("<repo-name>", repo),
                        description: `${permission} team for ${repo}`,
                        maintainers: [],
                        repo_names: [repo],
                        privacy: "closed",
                    });
                } catch (error) {
                    if (error.message.indexOf("must be unique") > -1) {
                        // name exists, grab the team instead
                        let existingTeam = await client.teams.getByName({
                            org: input.organization,
                            team_slug: team.replace("<repo-name>", repo),
                        });
                        newTeams[permission] = existingTeam["data"];
                    } else {
                        throw error;
                    }
                }

                await client.teams.addOrUpdateRepoPermissionsInOrg({
                    org: input.organization,
                    owner: input.organization,
                    team_slug: team.replace("<repo-name>", repo),
                    repo,
                    permission,
                });
            }

            // for each team get users and add to proper team from before and remove from repo
            for (const team of currentTeams) {
                let stuff = Object.entries(newTeams).filter(
                    ([key, newTeam]) => {
                        //console.log( `Key: ${key} Value: ${JSON.stringify(newTeam)} team: ${JSON.stringify(team)}`);
                        return newTeam.id === team.id;
                    }
                );
                if (stuff.length < 1) {
                    options = client.teams.listMembersInOrg.endpoint.merge({
                        org: input.organization,
                        team_slug: team.name,
                        per_page: 100,
                    });
                    const members = await client.paginate(options);

                    for (let member of members) {
                        client.teams.addOrUpdateMembershipForUserInOrg({
                            org: input.organization,
                            team_slug:
                                newTeams[getPermission(team.permission)].name,
                            username: member.login,
                        });
                    }

                    // Finally remove the team from the repo
                    await client.teams.removeRepoInOrg({
                        org: input.organization,
                        team_slug: team.name,
                        owner: input.organization,
                        repo,
                    });
                } else {
                    console.log(`Skipping ${team.name}`);
                }
            }
        }
    } catch (error) {
        console.log(error);
    }
}

if (require.main == module) {
    main();
}

module.exports = main;
