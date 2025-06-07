use crate::bot_lib::meme_generator;
use crate::{Context, Error};
use ::serenity::all::colours;
use poise::serenity_prelude as serenity;
use tracing::debug;

async fn autocomplete_meme_template<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> serenity::CreateAutocompleteResponse<'a> {
    let partial_lower = partial.to_lowercase();
    let templates = meme_generator::get_meme_template_files().unwrap_or_default();
    let choices: Vec<serenity::AutocompleteChoice<'a>> = templates
        .into_iter()
        .filter(move |template| {
            let template_lower = template.to_lowercase();
            template_lower.starts_with(&partial_lower)
        })
        .map(|name| serenity::AutocompleteChoice::new(name.clone(), name))
        .collect();

    serenity::CreateAutocompleteResponse::new().set_choices(choices)
}

/// Generate a meme with the specified template and text
#[poise::command(
    slash_command,
    description_localized("en-US", "Generate a meme with the specified template and text.")
)]
pub async fn generate_meme(
    ctx: Context<'_>,
    #[description = "Name of the meme template file"]
    #[autocomplete = "autocomplete_meme_template"]
    template: String,
    #[description = "Text to display at the top of the meme"] top_text: String,
    #[description = "Text to display at the bottom of the meme"] bottom_text: String,
) -> Result<(), Error> {
    // Defer the response since meme generation might take a moment
    ctx.defer().await?;

    // Generate the meme
    match meme_generator::generate_meme_as_file_path(&template, &top_text, &bottom_text) {
        Ok(meme_path) => {
            // Create attachment from the file path
            let attachment = serenity::CreateAttachment::path(&meme_path)?;
            let filename = attachment.filename.clone();
            let embed = serenity::CreateEmbed::default()
                .title("Generated Meme")
                .description(format!("Template: `{}`", template))
                .colour(colours::branding::BLACK)
                .attachment(filename);

            // Send the meme as a response
            debug!(attachment = ?attachment, "Sending generated meme");
            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .attachment(attachment),
            )
            .await?;
        }
        Err(e) => {
            let error_msg = format!("Failed to generate meme: {}", e);
            ctx.send(
                poise::CreateReply::default()
                    .content(error_msg)
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}
