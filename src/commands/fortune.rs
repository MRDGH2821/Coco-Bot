use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde::Deserialize;

#[derive(Deserialize)]
struct FortuneCookie {
    text: String,
    source: Option<String>,
    url: Option<String>,
    numbers: Option<String>,
    _html: Option<String>,
}

/// Call the fortune cookie API
async fn call_fortune_api(mode: &str) -> Result<FortuneCookie, Error> {
    let client = reqwest::Client::new();
    let url = "https://api.viewbits.com/v1/fortunecookie";

    let response = client.get(url).query(&[("mode", mode)]).send().await?;

    let fortune: FortuneCookie = response.json().await?;
    Ok(fortune)
}

/// Create fortune embed for Discord
fn create_fortune_embed<'a>(fortune: &FortuneCookie, mode: &str) -> serenity::CreateEmbed<'a> {
    let mut embed = serenity::CreateEmbed::new();

    // Set title and color based on mode
    match mode {
        "today" => {
            embed = embed.title("🔮 Fortune Cookie of the Day").color(0x9400D3); // Purple color for daily fortune
        }
        "random" => {
            embed = embed.title("🥠 Your Fortune Cookie").color(0xFFD700); // Gold color for random fortune
        }
        _ => {
            embed = embed.title("🥠 Fortune Cookie").color(0xFFD700);
        }
    }

    // Add the fortune text as description
    embed = embed.description(fortune.text.clone());

    // Add lucky numbers as a field if available
    if let Some(numbers) = &fortune.numbers {
        embed = embed.field("🍀 Lucky Numbers", numbers.clone(), false);
    }

    // Add source as footer if available
    if let Some(source) = &fortune.source {
        embed = embed.footer(serenity::CreateEmbedFooter::new(format!(
            "Source: {}",
            source
        )));
    }

    // Add timestamp
    embed = embed.timestamp(serenity::Timestamp::now());

    embed
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a fortune cookie message."),
    user_cooldown = "20"
)]
pub async fn fortune(
    ctx: Context<'_>,
    #[description = "Type of fortune"]
    #[choices("random", "today")]
    mode: &'static str,
) -> Result<(), Error> {
    // Validate mode
    if !["random", "today"].contains(&mode) {
        ctx.say("❌ Invalid mode. Please choose 'random' or 'today'.")
            .await?;
        return Ok(());
    }

    // Show typing indicator since API calls might take time
    ctx.defer().await?;

    match call_fortune_api(&mode).await {
        Ok(fortune) => {
            let embed = create_fortune_embed(&fortune, &mode);
            let mut reply = poise::CreateReply::default().embed(embed);

            // Add a button for the URL if available
            if let Some(url) = &fortune.url {
                let button = serenity::CreateButton::new_link(url).label("🔗 View Source");

                let action_row = serenity::CreateActionRow::Buttons(vec![button].into());
                reply = reply.components(vec![action_row]);
            }

            ctx.send(reply).await?;
        }
        Err(e) => {
            tracing::error!("Fortune API error: {}", e);
            ctx.say("❌ Sorry, I couldn't fetch your fortune right now. The cosmic energies might be disrupted! 🔮").await?;
        }
    }

    Ok(())
}
