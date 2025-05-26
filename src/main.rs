use ::serenity::all::Token;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
mod commands;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        Token::from_env("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            ..Default::default()
        })
        .initialize_owners(true)
        // .setup(|ctx, _ready, framework| {
        //     Box::pin(async move {
        //         poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        //         Ok(Data {})
        //     })
        // })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
