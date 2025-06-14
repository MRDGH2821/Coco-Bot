use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde::Deserialize;

#[derive(Deserialize)]
struct UselessFact {
    text: String,
    source: Option<String>,
    url: Option<String>,
    #[allow(dead_code)]
    html: Option<String>,
}

/// Call the useless facts API
async fn call_facts_api(mode: &str) -> Result<UselessFact, Error> {
    let client = reqwest::Client::new();
    let url = "https://api.viewbits.com/v1/uselessfacts";

    let response = client.get(url).query(&[("mode", mode)]).send().await?;

    let fact: UselessFact = response.json().await?;
    Ok(fact)
}

/// Create fact embed for Discord
fn create_fact_embed<'a>(fact: &UselessFact, mode: &str) -> serenity::CreateEmbed<'a> {
    let mut embed = serenity::CreateEmbed::new();

    // Set title and color based on mode
    match mode {
        "today" => {
            embed = embed.title("🧠 Useless Fact of the Day").color(0x4682B4); // Steel blue color for daily fact
        }
        "random" => {
            embed = embed.title("🤓 Random Useless Fact").color(0x20B2AA); // Light sea green color for random fact
        }
        _ => {
            embed = embed.title("🤓 Useless Fact").color(0x20B2AA);
        }
    }

    // Add the fact text as description
    embed = embed.description(fact.text.clone());

    // Add source as footer if available
    if let Some(source) = &fact.source {
        embed = embed.footer(serenity::CreateEmbedFooter::new(format!(
            "Source: {}",
            source
        )));
    }

    // Add timestamp
    embed = embed.timestamp(serenity::Timestamp::now());

    embed
}
#[derive(Default, poise::ChoiceParameter)]
pub enum Mode {
    #[default]
    Random,
    Today,
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a useless fact."),
    user_cooldown = 20
)]
pub async fn fact(
    ctx: Context<'_>,
    #[description = "Type of fact"] mode: Option<Mode>,
) -> Result<(), Error> {
    // Validate mode
    // let mode = mode.unwrap_or("random");

    // Show typing indicator since API calls might take time
    ctx.defer().await?;
    let mode = match mode {
        Some(Mode::Today) => "today",
        _ => "random", // Default to random if not specified
    };
    match call_facts_api(&mode).await {
        Ok(fact) => {
            let embed = create_fact_embed(&fact, &mode);
            let mut reply = poise::CreateReply::default().embed(embed);

            // Add a button for the URL if available
            if let Some(url) = &fact.url {
                let button = serenity::CreateButton::new_link(url).label("🔗 View Source");

                let action_row = serenity::CreateActionRow::Buttons(vec![button].into());
                reply = reply.components(vec![action_row]);
            }

            ctx.send(reply).await?;
        }
        Err(e) => {
            tracing::error!("Facts API error: {}", e);
            ctx.say("❌ Sorry, I couldn't fetch a fact right now. The knowledge servers might be taking a break! 🤓").await?;
        }
    }

    Ok(())
}
