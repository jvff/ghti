use structopt::StructOpt;

/// Command-line arguments.
#[derive(Clone, Debug, StructOpt)]
pub struct Arguments {
    /// The GitHub user or organization that owns the repository.
    #[structopt(short, long)]
    owner: String,

    /// The GitHub repository.
    #[structopt(short, long)]
    repo: String,
}

fn main() {
    let arguments = Arguments::from_args();

    dbg!(arguments);
}
