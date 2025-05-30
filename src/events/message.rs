use crate::Error;
use poise::serenity_prelude as serenity;
use tracing::{debug, error, warn};

pub async fn message(ctx: serenity::Context, msg: serenity::Message) -> Result<(), Error> {
    // Ignore messages from bots
    if msg.author.bot() {
        return Ok(());
    }

    // Log all messages (optional - can be removed in production)
    debug!(
        user = %msg.author.name,
        user_id = %msg.author.id,
        content = %msg.content,
        "Received message"
    );

    // Example: React to messages containing "hello"
    if msg.content.to_lowercase().contains("hello") {
        if let Err(e) = msg.react(&ctx.http, '👋').await {
            error!(error = %e, "Failed to react to message");
        }
    }

    // Example: Auto-delete messages containing bad words
    let bad_words = vec!["spam", "badword"]; // Add your own filter
    for word in bad_words {
        if msg.content.to_lowercase().contains(word) {
            warn!(word = %word, user = %msg.author.name, "Bad word detected, deleting message");
            if let Err(e) = msg.delete(&ctx.http, Some("Bad word detected")).await {
                error!(error = %e, "Failed to delete message");
            }
            return Ok(());
        }
    }

    Ok(())
}
