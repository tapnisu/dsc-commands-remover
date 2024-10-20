# dsc-commands-remover

CLI tool to unregister all slash commands for a discord bot

## Installation

`cargo install dsc-commands-remover`

## Usage

`dsc-commands-remover -t YOUR_DISCORD_BOT_TOKEN -a APPLICATION_ID` global commands only

`dsc-commands-remover -t YOUR_DISCORD_BOT_TOKEN -a APPLICATION_ID -g GUILD_ID` guild commands only

`dsc-commands-remover -t YOUR_DISCORD_BOT_TOKEN -a APPLICATION_ID -y` for auto confirm
