use clap::Args;
use octocrab::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Args, Clone, Debug)]
pub struct ConsolidateTeams {
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

pub async fn exec(_octocrab: Octocrab, args: ConsolidateTeams) -> Result<(), Box<dyn Error>> {
    if args.example {
        let props = Properties {
            repositories: vec!["test string".to_string()],
            name: "Chris".to_string(),
        };
        println!("{}", json!(props).to_string());
    } else {
        println!("Section that deals with files");
    }

    Ok(())
}
