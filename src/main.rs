use {
    derive_more::{Display, Error},
    octocrab::OctocrabBuilder,
    std::env,
    structopt::StructOpt,
};

/// Command-line arguments.
#[derive(Clone, Debug, StructOpt)]
pub struct Arguments {
    /// The GitHub user or organization that owns the repository.
    #[structopt(short, long, requires("repo"))]
    owner: Option<String>,

    /// The GitHub repository.
    #[structopt(short, long, requires("owner"))]
    repo: Option<String>,

    /// The issue to clone.
    issue: u64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arguments = Arguments::from_args();
    let owner;
    let repo;

    if let (Some(owner_argument), Some(repo_argument)) = (arguments.owner, arguments.repo) {
        owner = owner_argument;
        repo = repo_argument;
    } else {
        // Since the repo parameter requires the owner parameter, if owner is not set then repo is
        // also not set
        let owner_and_repo = env::var("GITHUB_REPO").map_err(|_| Error::InvalidGitHubRepoEnvVar)?;
        let mut parts = owner_and_repo.split("/");

        owner = parts
            .next()
            .ok_or(Error::InvalidGitHubRepoEnvVar)?
            .to_owned();
        repo = parts
            .next()
            .ok_or(Error::InvalidGitHubRepoEnvVar)?
            .to_owned();

        if parts.next().is_some() {
            return Err(Error::InvalidGitHubRepoEnvVar);
        }
    }

    if let Some(token_value) = env::var_os("GITHUB_TOKEN") {
        let token = token_value
            .into_string()
            .map_err(|_| Error::InvalidGitHubTokenEnvVar)?;

        octocrab::initialise(OctocrabBuilder::new().personal_token(token))
            .map_err(Error::InitialiseOctocrab)?;
    }

    let issue = octocrab::instance()
        .issues(&owner, &repo)
        .get(arguments.issue)
        .await
        .map_err(Error::FetchIssue)?;

    dbg!(issue);

    Ok(())
}

/// Errors that can happen when running the program.
#[derive(Debug, Display, Error)]
pub enum Error {
    /// Failed to fetch GitHub issue.
    #[display(fmt = "Failed to fetch GitHub issue")]
    FetchIssue(octocrab::Error),

    /// Failed to initialise Octocrab instance to use GitHub API.
    #[display(fmt = "Failed to initialise Octocrab instance to use GitHub API")]
    InitialiseOctocrab(octocrab::Error),

    /// `GITHUB_TOKEN` environment variable is not a valid UTF-8 string.
    #[display(fmt = "Invalid GITUB_TOKEN environment variable")]
    InvalidGitHubTokenEnvVar,

    /// `GITHUB_REPO` environment variable is either empty or invalid.
    #[display(
        fmt = "Invalid GITUB_REPO environment variable, it must have the format `<owner>/<repo>`"
    )]
    InvalidGitHubRepoEnvVar,
}
