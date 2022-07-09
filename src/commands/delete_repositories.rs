use crate::DeleteRepositories;
use csv;
use octocrab::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

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
            Ok(result) => println!("Deleted repository {}/{}", organization, repository),
            Err(error) => println!(
                "Error deleting repository {}/{} {}",
                organization, repository, error
            ),
        }
    }
    Ok(())
}
