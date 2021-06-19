use {
    derive_more::{Display, Error, From},
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

    dbg!(owner);
    dbg!(repo);

    Ok(())
}

/// Errors that can happen when running the program.
#[derive(Clone, Debug, Display, Error, From)]
pub enum Error {
    /// `GITHUB_REPO` environment variable is either empty or invalid.
    #[display(
        fmt = "Invalid GITUB_REPO environment variable, it must have the format `<owner>/<repo>`"
    )]
    InvalidGitHubRepoEnvVar,
}
