mod message;
mod ready;

use poise::serenity_prelude as serenity;
use serenity::async_trait;

pub struct Handler;

#[async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, ctx: &serenity::Context, event: &serenity::FullEvent) {
        match event {
            serenity::FullEvent::Message { new_message, .. } => {
                let _ = message::message(ctx.clone(), new_message.clone()).await;
            }
            serenity::FullEvent::Ready { data_about_bot, .. } => {
                ready::ready(ctx.clone(), data_about_bot.clone()).await;
            }
            _ => {}
        }
    }
}
