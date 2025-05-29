use poise::serenity_prelude as serenity;
use crate::Error;

pub async fn message(ctx: serenity::Context, msg: serenity::Message) -> Result<(), Error> {
    // Ignore messages from bots
    if msg.author.bot {
        return Ok(());
    }

    // Log all messages (optional - can be removed in production)
    println!("Message from {}: {}", msg.author.name, msg.content);

    // Example: React to messages containing "hello"
    if msg.content.to_lowercase().contains("hello") {
        if let Err(e) = msg.react(&ctx.http, '👋').await {
            eprintln!("Failed to react to message: {:?}", e);
        }
    }

    // Example: Auto-delete messages containing bad words
    let bad_words = vec!["spam", "badword"]; // Add your own filter
    for word in bad_words {
        if msg.content.to_lowercase().contains(word) {
            if let Err(e) = msg.delete(&ctx.http).await {
                eprintln!("Failed to delete message: {:?}", e);
            }
            return Ok(());
        }
    }

    Ok(())
}
