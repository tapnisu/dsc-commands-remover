pub mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use dialoguer::Confirm;
use reqwest::{blocking::Client, header::AUTHORIZATION};
use std::process;

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    if !cli.yes
        && !Confirm::new()
            .with_prompt("Do you want to remove all of your commands?")
            .interact()
            .unwrap()
    {
        process::exit(1);
    }

    if let Err(err) = remove_commands(&cli.application_id, &cli.bot_token) {
        cmd.error(ErrorKind::InvalidValue, err).exit();
    }

    println!("Removed commands successfully!");

    Ok(())
}

/// Just a workaround
const EMPTY_JSON_ARRAY: &serde_json::Value = &serde_json::json!([]);

/// Pushes `[]` as commands to Discord's API route
fn remove_commands(application_id: &str, bot_token: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        application_id
    );

    let res = client
        .put(url)
        .header(AUTHORIZATION, format!("Bot {}", bot_token))
        .json(EMPTY_JSON_ARRAY)
        .send()?;

    res.error_for_status()?;

    Ok(())
}
