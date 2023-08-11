pub mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use dialoguer::Confirm;
use poise::{
    serenity_prelude::{self as serenity},
    Command,
};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}

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
        .setup(move |ctx, _ready, _framework| {
            Box::pin(async move {
                if cli.yes
                    || Confirm::new()
                        .with_prompt(format!(
                            "Do you want to remove all of your commands for {}?",
                            _ready.user.tag()
                        ))
                        .interact()?
                {
                    poise::builtins::register_globally(
                        ctx,
                        vec![].as_slice() as &[Command<Context<'static>, Error>],
                    )
                    .await?;

                    println!("Removed commands successfully!");
                }

                safe_exit(0);
            })
        });

    let result = framework.run().await;

    match result {
        Ok(_) => (),
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
    }

    Ok(())
}
