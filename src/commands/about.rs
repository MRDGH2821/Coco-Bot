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
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let repository = env!("CARGO_PKG_REPOSITORY");

    // Get build-time information
    let git_hash = env!("GIT_HASH");
    let git_branch = env!("GIT_BRANCH");
    let build_time = env!("BUILD_TIMESTAMP");

    // Get dependency versions (these are git dependencies, so we'll show their git hashes)
    let serenity_version = env!("SERENITY_VERSION");
    let serenity_hash = env!("SERENITY_GIT_HASH");
    let poise_version = env!("POISE_VERSION");
    let poise_hash = env!("POISE_GIT_HASH");

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
            env!("RUSTC_VERSION")
        ), true)
        .field("🔧 Build Info", format!(
            "**Built:** <t:{}:f> (<t:{}:R>)\n\
            **Target:** {}",
            Timestamp::parse(&build_time).unwrap().timestamp(),
            Timestamp::parse(&build_time).unwrap().timestamp(),
            env!("TARGET")
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
