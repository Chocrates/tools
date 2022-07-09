mod commands;
use clap::{Args, Parser, Subcommand};

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
    DeleteRepositories(DeleteRepositories),
}

#[derive(Args)]
struct DeleteRepositories {
    /// Path to CSV file with a single column containing repositories to delete in format
    /// "owner/repository"
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DeleteRepositories(delete_repositories) => {
            commands::delete_repositories::run();
        }
    }
}
