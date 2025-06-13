use crate::Error;

/// Call the pickup/breakup line API
async fn get_random_line(line_type: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.jcwyt.com/{}", line_type);

    let response = client.get(&url).send().await?;

    let line_text = response.text().await?;
    let trimmed_line = line_text.trim().to_string();

    if trimmed_line.is_empty() {
        return Err("API returned an empty line".into());
    }

    Ok(trimmed_line)
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a pickup or breakup line."),
    user_cooldown = "3"
)]
pub async fn pickup_line(
    ctx: crate::Context<'_>,
    #[description = "Which type of lines to show"]
    #[choices("pickup", "breakup")]
    line_type: &'static str,
) -> Result<(), Error> {
    match get_random_line(&line_type).await {
        Ok(line) => {
            ctx.say(line).await?;
        }
        Err(e) => {
            tracing::error!("Pickup line API error: {}", e);

            let error_reply = poise::CreateReply::default()
                .content("❌ Sorry, I couldn't fetch a line right now. https://api.jcwyt.com/ might be down! 💔\n\nTry again later.")
                .ephemeral(true);

            ctx.send(error_reply).await?;
        }
    }

    Ok(())
}
