mod commands;
mod utility;
use clap::{Parser, Subcommand};
use commands::*;
use octocrab::*;
use std::error::Error;

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
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
    /// Get GitHub App Installation Token for specified organization
    GetAppToken(get_app_token::GetAppToken),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::DeleteRepositories(delete_repository_args) => {
            delete_repositories::exec(delete_repository_args.clone()).await?;
        }
        Commands::ConsolidateTeams(consolidate_teams_args) => {
            consolidate_teams::exec(consolidate_teams_args.clone()).await?
        }
        Commands::TransferRepositories(transfer_repository_args) => {
            transfer_repositories::exec(transfer_repository_args.clone()).await?
        }
        Commands::EnableActions(enable_actions_args) => {
            enable_actions::exec(enable_actions_args.clone()).await?
        }
        Commands::GetAppToken(get_app_token_args) => {
            get_app_token::exec(get_app_token_args.clone()).await?
        }
    }

    Ok(())
}
