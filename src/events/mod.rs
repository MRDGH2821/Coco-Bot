mod message;
mod ready;

use poise::serenity_prelude as serenity;

// Event handler struct that implements EventHandler
pub struct EventHandler;

#[serenity::async_trait]
impl serenity::EventHandler for EventHandler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        ready::ready(ctx, ready).await;
    }

    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        if let Err(e) = message::message(ctx, msg).await {
            eprintln!("Error in message event: {:?}", e);
        }
    }
}
