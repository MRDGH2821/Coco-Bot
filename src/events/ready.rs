use poise::serenity_prelude as serenity;
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

    // Set bot activity/status
    let activity = serenity::ActivityData::playing("with Rust!");
    let status = serenity::OnlineStatus::Online;

    ctx.set_presence(Some(activity), status);
}
