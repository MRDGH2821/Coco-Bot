use poise::serenity_prelude as serenity;

pub async fn ready(ctx: serenity::Context, ready: serenity::Ready) {
    println!("Bot is ready! Logged in as {}", ready.user.name);
    println!("Connected to {} guilds", ready.guilds.len());

    // Set bot activity/status
    let activity = serenity::ActivityData::playing("with Rust!");
    let status = serenity::OnlineStatus::Online;

    ctx.set_presence(Some(activity), status);
}
