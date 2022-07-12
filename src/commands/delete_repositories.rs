use clap::Args;
use csv;
use octocrab::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Args, Clone, Debug)]
pub struct DeleteRepositories {
    /// Path to CSV file with a single column containing repositories to delete in format
    /// "owner/repository"
    #[clap(short, long, value_parser)]
    file: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde()]
    repository: String,
}

pub async fn exec(octocrab: Octocrab, args: DeleteRepositories) -> Result<(), Box<dyn Error>> {
    let file = File::open(args.file)?;
    let mut csv_reader = csv::Reader::from_reader(file);
    for result in csv_reader.deserialize() {
        let record: Record = result.unwrap();
        let vec = record.repository.split("/").collect::<Vec<&str>>();
        let organization = vec[0];
        let repository = vec[1];

        match octocrab.repos(organization, repository).delete().await {
            Ok(_) => println!("Deleted repository {}/{}", organization, repository),
            Err(error) => println!(
                "Error deleting repository {}/{} {}",
                organization, repository, error
            ),
        }
    }
    Ok(())
}
