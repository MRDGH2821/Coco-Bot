use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FactResponse {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random fun fact.")
)]
pub async fn fact(ctx: Context<'_>) -> Result<(), Error> {
    // Send a "thinking" response first since API calls can take time
    ctx.defer().await?;

    // Fetch a random fact from the UselessFacts API
    let response = reqwest::get("https://uselessfacts.jsph.pl/api/v2/facts/random?language=en")
        .await?
        .json::<FactResponse>()
        .await?;

    // Create an embed for the fact
    let embed = serenity::CreateEmbed::default()
        .title("🧠 Random Fact")
        .description(&response.text)
        .field("Source", &response.source, true)
        .color(serenity::Color::from_rgb(70, 130, 180)) // Steel blue color
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Fact ID: {}",
            response.id
        )))
        .timestamp(serenity::Timestamp::now());

    // Create a button to link to the source
    let source_button =
        serenity::CreateButton::new_link(&response.source_url).label("🔗 View Source");

    let permalink_button =
        serenity::CreateButton::new_link(&response.permalink).label("📋 Permalink");

    let action_row =
        serenity::CreateActionRow::Buttons(vec![source_button, permalink_button].into());

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .components(vec![action_row]),
    )
    .await?;

    Ok(())
}
