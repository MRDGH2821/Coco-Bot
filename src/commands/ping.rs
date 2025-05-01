use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = ctx.say("Pong!").await?;

    let res_timestamp = response
        .message()
        .await
        .unwrap()
        .into_owned()
        .timestamp
        .timestamp();

    let ctx_timestamp = ctx.created_at().timestamp();

    let latency = res_timestamp - ctx_timestamp;

    ctx.say(format!("Pong! Latency: {}ms", latency)).await?;
    Ok(())
}
