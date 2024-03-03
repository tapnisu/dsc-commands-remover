pub mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use dialoguer::Confirm;
use dsc_commands_remover::remove_commands;
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
