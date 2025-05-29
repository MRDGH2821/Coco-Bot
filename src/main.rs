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
    let mut intents = serenity::GatewayIntents::non_privileged();
    intents.insert(serenity::GatewayIntents::MESSAGE_CONTENT);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
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
