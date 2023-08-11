use poise::{serenity_prelude as serenity, Command};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn oh_hi(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("oh, hi!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![oh_hi()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
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

    framework.run().await.unwrap();
}
