use clap::Args;
use csv;
use octocrab::*;
use serde::*;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::{thread, time};

#[derive(Args, Clone, Debug)]
pub struct TransferRepositories {
    /// Path to CSV file with a single column containing repositories to delete in format
    /// "owner/repository"
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Organization in which to transfer the repositories
    #[clap(short, long, value_parser)]
    organization: Option<String>,

    /// Prints an example csv file to stdio and then exits
    /// This data can be saved and edited to create your input file
    #[clap(short, long, value_parser)]
    example: bool,

    /// Enables actions on the transfered repositories
    #[clap(short = 'a', long, value_parser)]
    enable_actions: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde()]
    repository: String,
}

#[derive(Debug, Clone)]
enum TeamPermissions {
    Pull,
    Push,
    Maintain,
    Triage,
    Admin,
}

impl FromStr for TeamPermissions {
    type Err = ();
    fn from_str(input: &str) -> Result<TeamPermissions, Self::Err> {
        match input.to_lowercase().as_str() {
            "read" => Ok(TeamPermissions::Pull),
            "write" => Ok(TeamPermissions::Push),
            "maintain" => Ok(TeamPermissions::Maintain),
            "triage" => Ok(TeamPermissions::Triage),
            "admin" => Ok(TeamPermissions::Admin),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TeamPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
struct Teams {
    name: String,
    slug: String,
    permissions: TeamPermissions,
    members: Vec<models::User>,
}

pub async fn exec(oc: Octocrab, args: TransferRepositories) -> Result<(), Box<dyn Error>> {
    if args.example {
        println!("repository\norganization/repository");
    } else {
        let path = args
            .file
            .ok_or_else(|| String::from("File is required when example is false"))?;
        let target_organization = args
            .organization
            .ok_or_else(|| String::from("Organization is required when example is false"))?;
        let file = File::open(path)?;
        let mut csv_reader = csv::Reader::from_reader(file);
        for result in csv_reader.deserialize() {
            let record: Record = result.unwrap();
            let vec = record.repository.split("/").collect::<Vec<&str>>();
            let organization = vec[0];
            let repository = vec[1];
            let mut teams = Vec::<Teams>::new();

            // Get all teams with access to repository
            let response: Vec<models::teams::Team> = oc
                .get(
                    oc.absolute_url(format!("/repos/{}/{}/teams", organization, repository))?,
                    None::<&()>,
                )
                .await?;

            for t in response.iter() {
                let permission = match TeamPermissions::from_str(&t.permission) {
                    Ok(permission) => permission,
                    Err(_) => {
                        println!("Invalid permission {} for team {}", t.permission, t.name);
                        TeamPermissions::Pull
                    }
                };

                let mut team = Teams {
                    name: t.name.clone(),
                    slug: t.slug.clone(),
                    permissions: permission,
                    members: Vec::<models::User>::new(),
                };

                let members: Vec<models::User> = oc
                    .get(
                        oc.absolute_url(format!(
                            "/orgs/{}/teams/{}/members",
                            organization, t.slug
                        ))?,
                        None::<&()>,
                    )
                    .await?;
                team.members = members.clone();
                teams.push(team.clone());
            }

            let mut new_teams = Vec::<models::teams::Team>::new();
            for t in teams.iter() {
                let new_team: models::teams::Team;

                // determine if team with this name exists in new org
                match oc.teams(&target_organization).get(&t.slug).await {
                    Ok(team) => {
                        new_team = team;
                    } // don't need to create the team
                    Err(error) => match error {
                        octocrab::Error::GitHub {
                            ref source,
                            backtrace: _,
                        } => {
                            if source.message == "Not Found" {
                                new_team = oc
                                    .teams(&target_organization)
                                    .create(&t.name)
                                    .privacy(params::teams::Privacy::Closed) // Make team public
                                    .send()
                                    .await
                                    .expect("Team Creation Failed");
                            } else {
                                panic!("Unknown error: {}", &error);
                            }
                        }
                        _ => panic!("Unknown error: {}", &error),
                    },
                };

                // add users to team
                for user in t.members.iter() {
                    let body = reqwest::Body::from("{\"role\":\"member\"}");

                    match oc
                        .request_builder(
                            oc.absolute_url(format!(
                                "/orgs/{}/teams/{}/memberships/{}",
                                target_organization, new_team.slug, user.login
                            ))?,
                            reqwest::Method::PUT,
                        )
                        .body(body)
                        .send()
                        .await
                    {
                        // Users that are not in the org will still return an HTTP 200, so all errors
                        // are going to be unrecoverable and thrown to the user
                        Ok(_) => {}
                        Err(error) => {
                            panic!("Unknown error {}", &error);
                        }
                    }
                }

                new_teams.push(new_team);
            }

            let team_ids = new_teams
                .iter()
                .map(|t| format!("{}", t.id))
                .collect::<Vec<String>>()
                .join(",");

            // Transfer Team to new org
            // Teams will not have proper permissions
            let body = reqwest::Body::from(format!(
                "{{\"new_owner\":\"{}\", \"team_ids\":[{}]}}",
                target_organization, &team_ids
            ));

            match oc
                .request_builder(
                    oc.absolute_url(format!("/repos/{}/{}/transfer", organization, repository))?,
                    reqwest::Method::POST,
                )
                .body(body)
                .send()
                .await
            {
                Ok(_) => {
                    println!("Transferred: {}/{}", target_organization, repository);
                }
                Err(error) => {
                    panic!("Unknown error: {}", &error);
                }
            }

            if args.enable_actions {
                let one_second = time::Duration::from_millis(1000);

                thread::sleep(one_second);
                // Enable Actions
                let body = reqwest::Body::from("{\"enabled\": true}");
                match oc
                    .request_builder(
                        oc.absolute_url(format!(
                            "/repos/{}/{}/actions/permissions",
                            target_organization, repository
                        ))?,
                        reqwest::Method::PUT,
                    )
                    .body(body)
                    .send()
                    .await
                {
                    // Users that are not in the org will still return an HTTP 200, so all errors
                    // are going to be unrecoverable and thrown to the user
                    Ok(res) => {
                        println!("{:?}", res);
                    }
                    Err(error) => {
                        panic!("Unknown error {}", &error);
                    }
                }
            }
        }
    }
    Ok(())
}
