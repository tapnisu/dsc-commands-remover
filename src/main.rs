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

/// Terminates the program with the specified exit code
pub fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}

/// Just an empty command
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
        })
        .build();

    let client =
        serenity::ClientBuilder::new(cli.token, serenity::GatewayIntents::non_privileged())
            .framework(framework)
            .await;

    match client.unwrap().start().await {
        Ok(_) => (),
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
    }

    Ok(())
}
