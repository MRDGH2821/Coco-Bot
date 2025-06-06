use crate::{Context, Error};

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random piece of advice.")
)]
pub async fn advice(ctx: Context<'_>) -> Result<(), Error> {
    let response = reqwest::get("https://api.adviceslip.com/advice")
        .await?
        .json::<serde_json::Value>()
        .await?;
    let advice = response["slip"]["advice"]
        .as_str()
        .unwrap_or("No advice found");

    ctx.say(advice).await?;
    Ok(())
}
