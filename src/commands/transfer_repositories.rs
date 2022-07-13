use clap::Args;
use csv;
use octocrab::*;
use reqwest::Method;
use reqwest::Url;
use serde::*;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

#[derive(Args, Clone, Debug)]
pub struct TransferRepositories {
    /// Path to CSV file with a single column containing repositories to delete in format
    /// "owner/repository"
    #[clap(short, long, value_parser)]
    file: String,

    /// Organization in which to transfer the repositories
    #[clap(short, long, value_parser)]
    organization: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde()]
    repository: String,
}

#[derive(Debug, Clone)]
enum TeamPermissions {
    Pull,
}

impl FromStr for TeamPermissions {
    type Err = ();
    fn from_str(input: &str) -> Result<TeamPermissions, Self::Err> {
        match input.to_lowercase().as_str() {
            "pull" => Ok(TeamPermissions::Pull),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Teams {
    name: String,
    slug: String,
    permissions: TeamPermissions,
    members: Vec<models::User>,
}

// trait Empty<T> {
//     fn empty() -> T;
// }

// impl Empty<models::teams::Team> for models::teams::Team {
//     fn empty() -> models::teams::Team {
//         models::teams::Team {
//             id: models::TeamId::from(1),
//             node_id: String::default(),
//             url: Url::parse("https://example.com/").unwrap(),
//             html_url: Url::parse("https://example.com/").unwrap(),
//             name: String::default(),
//             slug: String::default(),
//             description: None,
//             privacy: String::default(),
//             permission: String::default(),
//             members_url: Url::parse("https://example.com/").unwrap(),
//             repositories_url: Url::parse("https://example.com/").unwrap(),
//             members_count: None,
//             repos_count: None,
//             created_at: None,
//             updated_at: None,
//             organization: None,
//         }
//     }
// }

pub async fn exec(oc: Octocrab, args: TransferRepositories) -> Result<(), Box<dyn Error>> {
    let file = File::open(args.file)?;
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
            let mut team = Teams {
                name: t.name.clone(),
                slug: t.slug.clone(),
                permissions: TeamPermissions::from_str(&t.permission).unwrap(),
                members: Vec::<models::User>::new(),
            };

            let members: Vec<models::User> = oc
                .get(
                    oc.absolute_url(format!("/orgs/{}/teams/{}/members", organization, t.slug))?,
                    None::<&()>,
                )
                .await?;
            team.members = members.clone();
            teams.push(team.clone());
        }

        // Transfer Team to new org
        // Do this without the new teams so we can make sure to add them with proper permission
        let body = reqwest::Body::from(format!("{{\"new_owner\":\"{}\"}}", args.organization));
        match oc
            .request_builder(
                oc.absolute_url(format!("/repos/{}/{}/transfer", organization, repository))?,
                reqwest::Method::POST,
            )
            .body(body)
            .send()
            .await
        {
            Ok(_) => {}
            Err(error) => {
                panic!("Unknown error: {}", &error);
            }
        }

        // for team in teams
        for t in teams.iter() {
            let mut new_team: models::teams::Team;

            // determine if team with this name exists in new org
            match oc.teams(&args.organization).get(&t.slug).await {
                Ok(team) => {
                    new_team = team;
                    println!("Found an existing team");
                } // don't need to create the team
                Err(error) => match error {
                    octocrab::Error::GitHub {
                        ref source,
                        ref backtrace,
                    } => {
                        if source.message == "Not Found" {
                            new_team = oc
                                .teams(&args.organization)
                                .create(&t.name)
                                .privacy(params::teams::Privacy::Closed) // Make team public
                                .send()
                                .await
                                .expect("Team Creation Failed");
                            println!("Created the new team!");
                        } else {
                            panic!("Unknown error: {}", &error);
                        }
                    }
                    _ => panic!("Unknown error: {}", &error),
                },
            };

            // add users to team
            for user in t.members.iter() {
                println!("User: {}", user.login);
                let body = reqwest::Body::from("{\"role\":\"member\"}");

                match oc
                    .request_builder(
                        oc.absolute_url(format!(
                            "/orgs/{}/teams/{}/memberships/{}",
                            args.organization, t.slug, user.login
                        ))?,
                        reqwest::Method::PUT,
                    )
                    .body(body)
                    .send()
                    .await
                {
                    // Users that are not in the org will still return an HTTP 200, so all errors
                    // are going to be unrecoverable and thrown to the user
                    Ok(res) => {}
                    Err(error) => {
                        panic!("Unknown error {}", &error);
                    }
                }
            }
        }
    }
    Ok(())
}
