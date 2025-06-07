use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Mentionable};

#[poise::command(
    slash_command,
    description_localized("en-US", "Let me Google that for you or someone else")
)]
pub async fn let_me_google_that(
    ctx: Context<'_>,
    #[description = "What should I google?"] search: String,
    #[description = "Whom should I ping to look at the result?"] target: Option<serenity::Member>,
) -> Result<(), Error> {
    // URL encode the search query
    let encoded_search = urlencoding::encode(&search);

    // Create the LMGTFY URL
    let lmgtfy_url = format!("https://letmegooglethat.com/?q={}", encoded_search);

    // Format the response message based on whether target is provided
    let response = if let Some(target_member) = target {
        format!("{}, here you go: {}", target_member.mention(), lmgtfy_url)
    } else {
        format!("Here you go: {}", lmgtfy_url)
    };

    ctx.say(response).await?;
    Ok(())
}
