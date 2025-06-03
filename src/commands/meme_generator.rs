use crate::bot_lib::meme_generator;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

fn autocomplete_meme_template<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice<'a>> {
    let partial_lower = partial.to_lowercase();
    let templates = meme_generator::get_meme_template_files().unwrap_or_default();
    templates
        .into_iter()
        .filter(move |template| {
            let template_lower = template.to_lowercase();
            template_lower.starts_with(&partial_lower)
        })
        .map(|name| serenity::AutocompleteChoice::new(name.clone(), name))
}

/// Generate a meme with the specified template and text
#[poise::command(slash_command)]
pub async fn meme_generator(
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

            // Send the meme as a response
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Here's your meme using template: `{}`", template))
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

/// List available meme templates
#[poise::command(slash_command)]
pub async fn meme_templates(ctx: Context<'_>) -> Result<(), Error> {
    match meme_generator::get_meme_template_files() {
        Ok(templates) => {
            if templates.is_empty() {
                ctx.say("No meme templates found!").await?;
            } else {
                let template_list = templates.join("\n• ");
                let response = format!("**Available meme templates:**\n• {}", template_list);

                // Split the response if it's too long for Discord
                if response.len() > 2000 {
                    let template_strings: Vec<&str> =
                        templates.iter().map(|s| s.as_str()).collect();
                    let chunked_strings: Vec<String> = template_strings
                        .chunks(20)
                        .map(|chunk| chunk.join("\n• "))
                        .collect();
                    let chunks: Vec<&str> = chunked_strings.iter().map(|s| s.as_str()).collect();

                    for (i, chunk) in chunks.iter().enumerate() {
                        let content = if i == 0 {
                            format!("**Available meme templates:**\n• {}", chunk)
                        } else {
                            format!("• {}", chunk)
                        };
                        ctx.say(content).await?;
                    }
                } else {
                    ctx.say(response).await?;
                }
            }
        }
        Err(e) => {
            ctx.say(format!("Failed to list templates: {}", e)).await?;
        }
    }
    Ok(())
}
