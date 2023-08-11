pub mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use poise::{
    serenity_prelude::{self as serenity},
    Command,
};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// none
#[poise::command(slash_command, prefix_command)]
async fn none(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![none()],
            ..Default::default()
        })
        .token(cli.token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    vec![].as_slice() as &[Command<Context<'static>, Error>],
                )
                .await?;

                Ok(Data {})
            })
        });

    let result = framework.run().await;

    match result {
        Ok(_) => (),
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
    }

    Ok(())
}
