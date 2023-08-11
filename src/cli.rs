use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Discord token
    #[clap(short, long, num_args = 1.., required=true)]
    pub token: String,

    /// Remove all commands without confirmation
    #[clap(short, long, action)]
    pub yes: bool,
}
