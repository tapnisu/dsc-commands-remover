use reqwest::{header::AUTHORIZATION, Client};

/// Just an empty serde_json array
pub const EMPTY_JSON_ARRAY: &serde_json::Value = &serde_json::json!([]);

/// Pushes `[]` as commands to Discord's API route
pub async fn remove_commands(application_id: &str, bot_token: &str, guild_id: &Option<String>) -> Result<(), reqwest::Error> {
    let url;

    if guild_id.is_some() {
        url = format!(
            "https://discord.com/api/v10/applications/{}/guilds/{}/commands",
            application_id,
            guild_id.as_ref().unwrap(),
        );
    }else{
        url = format!(
            "https://discord.com/api/v10/applications/{}/commands",
            application_id
        );
    }

    call_command_api(&url, bot_token).await
}

pub async fn call_command_api(url: &String, bot_token: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let res = client
        .put(url)
        .header(AUTHORIZATION, format!("Bot {}", bot_token))
        .json(EMPTY_JSON_ARRAY)
        .send()
        .await?;

    res.error_for_status()?;
    Ok(())
}
