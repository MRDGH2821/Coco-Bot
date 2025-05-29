mod message;
mod ready;

use crate::{Data, Error};
use poise::FrameworkContext;
use poise::serenity_prelude as serenity;

// Event handler function for poise framework
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            ready::ready(ctx.clone(), data_about_bot.clone()).await;
        }
        serenity::FullEvent::Message { new_message } => {
            if let Err(e) = message::message(ctx.clone(), new_message.clone()).await {
                eprintln!("Error in message event: {:?}", e);
            }
        }
        _ => {}
    }
    Ok(())
}
