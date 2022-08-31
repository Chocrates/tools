use clap::Args;
use octocrab::*;
use std::error::Error;

#[derive(Args, Clone, Debug)]
pub struct GetIssues {
    /// Owner that owns the repos
    #[clap(short, long, value_parser)]
    owner: String,
    /// Repository that contains the issues
    #[clap(short, long, value_parser)]
    repo: String,
    /// Tag to filter issues by
    #[clap(short, long, value_parser)]
    tag: String,
}

pub async fn exec(octocrab: Octocrab, args: GetIssues) -> Result<(), Box<dyn Error>> {
    let res: Vec<models::issues::Issue> = octocrab
        .get(
            octocrab.absolute_url(format!(
                "/repos/{}/{}/issues?state=all&per_page=100",
                args.owner, args.repo
            ))?,
            None::<&()>,
        )
        .await?;
    println!("{:?}", res);
    Ok(())
}
