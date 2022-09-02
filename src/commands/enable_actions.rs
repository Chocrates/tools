use clap::Args;
use csv;
use octocrab::*;
use serde::*;
use std::error::Error;
use std::fs::File;

#[derive(Args, Clone, Debug)]
pub struct EnableActions {
    /// Path to CSV file with a single column containing repositories to enable in format
    /// "owner/repository"
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Prints an example csv file to stdio and then exits
    /// This data can be saved and edited to create your input file
    #[clap(short, long, value_parser)]
    example: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde()]
    repository: String,
}

pub async fn exec(oc: Octocrab, args: EnableActions) -> Result<(), Box<dyn Error>> {
    if args.example {
        println!("repository\norganization/repository");
    } else {
        let path = args
            .file
            .ok_or_else(|| String::from("File is required when example is false"))?;
        let file = File::open(path)?;
        let mut csv_reader = csv::Reader::from_reader(file);
        for result in csv_reader.deserialize() {
            let record: Record = result.unwrap();
            let vec = record.repository.split("/").collect::<Vec<&str>>();
            let organization = vec[0];
            let repository = vec[1];
            let body = reqwest::Body::from("{\"enabled\": true}");
            match oc
                .request_builder(
                    oc.absolute_url(format!(
                        "/repos/{}/{}/actions/permissions",
                        organization, repository
                    ))?,
                    reqwest::Method::PUT,
                )
                .body(body)
                .send()
                .await
            {
                // Users that are not in the org will still return an HTTP 200, so all errors
                // are going to be unrecoverable and thrown to the user
                Ok(_res) => {
                    println!("Actions enabled for {}/{}", organization, repository);
                }
                Err(error) => {
                    println!("Unknown error {}", &error);
                }
            }
        }
    }
    Ok(())
}
