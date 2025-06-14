use crate::commands;
use poise::{builtins::register_globally, serenity_prelude as serenity};
use tracing::info;

pub async fn ready(ctx: serenity::Context, ready: serenity::Ready) {
    info!(
        "Bot is ready! Logged in as {}#{:?}",
        ready.user.name,
        ready.user.discriminator.unwrap()
    );
    info!(
        guild_count = ready.guilds.len(),
        "Connected to {} guilds",
        ready.guilds.len()
    );

    let all_commands = commands::all_commands();
    register_globally(&ctx.http, &all_commands)
        .await
        .expect("Failed to register commands globally");

    info!(
        command_count = all_commands.len(),
        "Commands registered globally"
    );

    // Set bot activity/status
    let activity = serenity::ActivityData::playing("with Rust!");
    let status = serenity::OnlineStatus::Online;

    ctx.set_presence(Some(activity), status);
}
