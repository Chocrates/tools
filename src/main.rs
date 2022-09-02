mod commands;
use clap::{Parser, Subcommand};
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
    /// Consolidates all users in all with access to a repository to the specified teams in a properties file
    ConsolidateTeams(consolidate_teams::ConsolidateTeams),
    /// Transfer repositories contained in csv file to new org, including teams and members
    /// NOTE: Team permissions are not maintained and will be transfered as read
    /// See: https://docs.github.com/en/rest/repos/repos#transfer-a-repository
    TransferRepositories(transfer_repositories::TransferRepositories),
    /// Enable GitHub Actions on repositories contained in csv file
    EnableActions(enable_actions::EnableActions),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Build octocrab instance before we pass it to the subcommand
    let octocrab = Octocrab::builder()
        .personal_token(cli.token)
        .build()
        .expect("Unable to build Octocrab instance");

    match &cli.command {
        Commands::DeleteRepositories(delete_repository_args) => {
            delete_repositories::exec(octocrab, delete_repository_args.clone()).await?;
        }
        Commands::ConsolidateTeams(consolidate_teams_args) => {
            consolidate_teams::exec(octocrab, consolidate_teams_args.clone()).await?
        }
        Commands::TransferRepositories(transfer_repository_args) => {
            transfer_repositories::exec(octocrab, transfer_repository_args.clone()).await?
        }
        Commands::EnableActions(enable_actions_args) => {
            enable_actions::exec(octocrab, enable_actions_args.clone()).await?
        }
    }

    Ok(())
}
