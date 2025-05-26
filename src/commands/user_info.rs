use crate::{Context, Error};
use ::serenity::all::CreateEmbed;
use poise::{CreateReply, serenity_prelude as serenity};

#[poise::command(slash_command, prefix_command)]
pub async fn user_info(
    ctx: Context<'_>,
    #[description = "Selected user"] guild_member: Option<serenity::Member>,
) -> Result<(), Error> {
    let member = if let Some(member) = guild_member {
        member
    } else {
        ctx.author_member().await.unwrap().into_owned()
    };

    let created_at = member.user.id.created_at().timestamp();
    let joined_at = member.joined_at.unwrap().timestamp();

    let response = format!(
        "Joined Discord on: <t:{}:F> (<t:{}:R>)\nJoined this server on: <t:{}:F> (<t:{}:R>)",
        created_at, created_at, joined_at, joined_at,
    );

    let embed = CreateEmbed::default()
        .title(format!("About {}", member.user.name))
        .description(response)
        .timestamp(serenity::Timestamp::now())
        .color(serenity::Color::from_rgb(0, 255, 0))
        .thumbnail(member.user.avatar_url().unwrap_or_default());

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}
