use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Your Discord bot's token
    #[clap(short = 't', long = "token", name = "token")]
    pub bot_token: String,

    /// Your Discord bot's application ID
    #[clap(short, long)]
    pub application_id: String,

    /// Optional. Only removes guild commands. When omitted, only removes global commands.
    #[clap(short, long)]
    pub guild_id: Option<String>,

    /// Remove commands without confirmation
    #[clap(short, long, action)]
    pub yes: bool,
}
