mod commands;
use clap::{Args, Parser, Subcommand};
use commands::*;
use octocrab::*;
use std::error::Error;

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// GitHub Personal Access Token with access to Organization or Repositories
    #[clap(short, long, value_parser)]
    token: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete Repositories contained in csv file
    DeleteRepositories(delete_repositories::DeleteRepositories),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Build octocrab instance before we pass it to the subcommand
    let octocrab = Octocrab::builder()
        .personal_token(cli.token)
        .build()
        .expect("Unable to authenticate with token");

    match &cli.command {
        Commands::DeleteRepositories(delete_repositories) => {
            delete_repositories::exec(octocrab, delete_repositories.clone()).await?;
        }
    }

    Ok(())
}