use crate::{Context, Error};
use ::serenity::all::Timestamp;
use poise::serenity_prelude as serenity;

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get information about the bot.")
)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    let repository = std::env::var("CARGO_PKG_REPOSITORY").unwrap();

    // Get build-time information
    let git_hash = std::env::var("GIT_HASH").unwrap();
    let git_branch = std::env::var("GIT_BRANCH").unwrap();
    let build_time = std::env::var("BUILD_TIMESTAMP").unwrap();

    // Get dependency versions (these are git dependencies, so we'll show their git hashes)
    let serenity_version = std::env::var("SERENITY_VERSION").unwrap();
    let serenity_hash = std::env::var("SERENITY_GIT_HASH").unwrap();
    let poise_version = std::env::var("POISE_VERSION").unwrap();
    let poise_hash = std::env::var("POISE_GIT_HASH").unwrap();

    let serenity_info = if serenity_hash != "unknown" && serenity_version != "unknown" {
        format!("v{} (git: `{}`)", serenity_version, serenity_hash)
    } else {
        "Git (next branch)".to_string()
    };
    let poise_info = if poise_hash != "unknown" && poise_version != "unknown" {
        format!("v{} (git: `{}`)", poise_version, poise_hash)
    } else {
        "Git (serenity-next branch)".to_string()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!("About {}", to_title_case(&name.replace("-", " "))))
        .description(format!(
            "This bot is a Rust port of [KittyBot](https://github.com/olliequ/KittyBot) for the CS@unimelb Discord server.\n\n\
            **Version:** `{}`\n\
            **Git Hash:** [`{}`](https://github.com/MRDGH2821/Coco-Bot/tree/{})\n\
            **Git Branch:** [`{}`](https://github.com/MRDGH2821/Coco-Bot/tree/{})",
            version, git_hash, git_hash, git_branch,git_branch
        ))
        .field("🦀 Dependencies", format!(
            "**Serenity:** [{}](https://github.com/serenity-rs/serenity/tree/{})\n\
            **Poise:** [{}](https://github.com/serenity-rs/poise/tree/{})\n\
            **Rust:** {}",
            serenity_info,
            serenity_hash,
            poise_info,
            poise_hash,
            std::env::var("RUSTC_VERSION").unwrap()
        ), true)
        .field("🔧 Build Info", format!(
            "**Built:** <t:{}:f> (<t:{}:R>)\n\
            **Target:** {}",
            Timestamp::parse(&build_time).unwrap().timestamp(),
            Timestamp::parse(&build_time).unwrap().timestamp(),
            std::env::var("TARGET").unwrap()
        ), true)
        .color(serenity::Color::from_rgb(255, 192, 203)) // Pink color
        .footer(serenity::CreateEmbedFooter::new(format!("Built with Rust 🦀 • {} ({})", version, git_hash)))
        .timestamp(serenity::Timestamp::now());

    // Create link button for repository
    let github_button =
        serenity::CreateButton::new_link(repository).label("📂 Coco Bot Source Code");

    // You could also add more buttons here, for example:
    let kittybot_button = serenity::CreateButton::new_link("https://github.com/olliequ/KittyBot")
        .label("🐱 KittyBot Source Code");

    let action_row =
        serenity::CreateActionRow::Buttons(vec![github_button, kittybot_button].into());

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .components(vec![action_row]),
    )
    .await?;
    Ok(())
}
