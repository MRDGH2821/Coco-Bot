use dotenv::dotenv;
use poise::serenity_prelude as serenity;
mod commands;
mod events;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        std::env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            event_handler: events::EventHandler,
            ..Default::default()
        })
        .initialize_owners(true)
        .build();
    println!("Starting bot...");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    println!("Client created");
    if let Err(why) = client.unwrap().start().await {
        println!("Err with client: {:?}", why);
    }
}
