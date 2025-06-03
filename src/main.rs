use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tracing::{error, info};
mod bot_lib;
mod commands;
mod events;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let token = serenity::all::Token::from_env("DISCORD_TOKEN")
        .expect("Expected a DISCORD_TOKEN in the environment");
    let mut intents = serenity::GatewayIntents::non_privileged();
    intents.insert(serenity::GatewayIntents::MESSAGE_CONTENT);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            ..Default::default()
        })
        .initialize_owners(true)
        .build();
    info!("Starting bot...");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .await;
    info!("Client created");
    if let Err(why) = client.unwrap().start().await {
        error!(error = %why, "Error with client");
    }
}
