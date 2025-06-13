use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DadJokeResponse {
    id: String,
    joke: String,
    status: u32,
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random dad joke to brighten your day!")
)]
pub async fn dad_joke(ctx: Context<'_>) -> Result<(), Error> {
    // Send a "thinking" response first since API calls can take time
    ctx.defer().await?;

    // Create a reqwest client with the required headers
    let client = reqwest::Client::new();
    let response = client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            "Coco-Bot (https://github.com/MRDGH2821/Coco-Bot)",
        )
        .send()
        .await?
        .json::<DadJokeResponse>()
        .await?;

    // Create an embed for the dad joke
    let embed = serenity::CreateEmbed::default()
        .title("Here's your Dad Joke")
        .description(&response.joke)
        .color(serenity::Color::from_rgb(255, 165, 0)) // Orange color for humor
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Joke ID: {} | Powered by icanhazdadjoke.com",
            response.id
        )))
        .timestamp(serenity::Timestamp::now());

    // Create a button to link to the joke on the website
    let joke_link_button =
        serenity::CreateButton::new_link(format!("https://icanhazdadjoke.com/j/{}", response.id))
            .label("🔗 View on Website");

    let action_row = serenity::CreateActionRow::Buttons(vec![joke_link_button].into());

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .components(vec![action_row]),
    )
    .await?;

    Ok(())
}
