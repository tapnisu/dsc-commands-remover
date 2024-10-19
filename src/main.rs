pub mod cli;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use dialoguer::Confirm;
use dsc_commands_remover::remove_commands;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    let prompt;
    if cli.guild_id.is_some(){
        prompt = "Do you want to remove all of your guild commands?";
    }else{
        prompt = "Do you want to remove all of your global commands?";
    }

    if !cli.yes
        && !Confirm::new()
            .with_prompt(prompt)
            .interact()
            .unwrap()
    {
        process::exit(1);
    }

    if let Err(err) = remove_commands(&cli.application_id, &cli.bot_token, &cli.guild_id).await {
        cmd.error(ErrorKind::InvalidValue, err).exit();
    }

    if cli.guild_id.is_some(){
        println!("Removed guild commands successfully!");
    }else{
        println!("Removed global commands successfully!");
    }

}
