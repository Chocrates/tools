use crate::utility;
use clap::Args;
use octocrab::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Args, Clone, Debug)]
pub struct ConsolidateTeams {
    /// GitHub Personal Access Token with access to Organization or Repositories
    #[clap(short, long, value_parser)]
    token: Option<String>,

    /// Path to properties file containing run configuration
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Prints a sample, formatted properties JSON file to stdout
    #[clap(short, long, value_parser)]
    example: bool,
}

#[derive(Serialize, Deserialize)]
struct Properties {
    repositories: Vec<String>,
    name: String,
}

pub async fn exec(args: ConsolidateTeams) -> Result<(), Box<dyn Error>> {
    if args.example {
        let props = Properties {
            repositories: vec!["test string".to_string()],
            name: "Chris".to_string(),
        };
        println!("{}", json!(props).to_string());
    } else {
        let token = args.token.ok_or_else(|| {
            String::from("Personal Access Token is required when example is not used")
        })?;
        let _octocrab = utility::build_octocrab(token);
        println!("Section that deals with files");
    }

    Ok(())
}
