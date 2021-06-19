use {std::env, structopt::StructOpt};

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

fn main() {
    let arguments = Arguments::from_args();
    let owner;
    let repo;

    if let (Some(owner_argument), Some(repo_argument)) = (arguments.owner, arguments.repo) {
        owner = owner_argument;
        repo = repo_argument;
    } else {
        // Since the repo parameter requires the owner parameter, if owner is not set then repo is
        // also not set
        let owner_and_repo = env::var("GITHUB_REPO").expect("Missing GitHub repository");
        let mut parts = owner_and_repo.split("/");

        owner = parts
            .next()
            .expect("Empty GITHUB_REPO environment variable")
            .to_owned();
        repo = parts
            .next()
            .expect("Incorrect GITHUB_REPO environment variable, must be <owner>/<repo>")
            .to_owned();

        assert!(
            parts.next().is_none(),
            "Incorrect GITHUB_REPO environment variable, must be <owner>/<repo>"
        );
    }

    dbg!(owner);
    dbg!(repo);
}
