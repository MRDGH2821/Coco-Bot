use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serde_json::Value;
use serenity::all::{CreateActionRow, CreateButton};

/// Autocomplete function for comic numbers
async fn comic_autocomplete<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> serenity::CreateAutocompleteResponse<'a> {
    let mut choices = Vec::new();

    if partial.is_empty() {
        // If partial is empty, suggest some popular comic numbers
        choices.push(serenity::AutocompleteChoice::new("353 (Python)", "353"));
        choices.push(serenity::AutocompleteChoice::new("927 (Standards)", "927"));
        choices.push(serenity::AutocompleteChoice::new("386 (Duty Calls)", "386"));
        choices.push(serenity::AutocompleteChoice::new("303 (Compiling)", "303"));
        choices.push(serenity::AutocompleteChoice::new(
            "1205 (Is It Worth the Time?)",
            "1205",
        ));
        choices.push(serenity::AutocompleteChoice::new(
            "1053 (Ten Thousand)",
            "1053",
        ));
        choices.push(serenity::AutocompleteChoice::new(
            "2347 (Dependency)",
            "2347",
        ));
    } else if let Ok(num) = partial.parse::<u32>() {
        // If user typed a number, suggest that number and nearby ones
        if num > 0 && num <= 3000 {
            // Reasonable upper bound for xkcd comics
            choices.push(serenity::AutocompleteChoice::new(
                format!("Comic #{}", num),
                num.to_string(),
            ));
        }

        // Suggest some nearby numbers
        for offset in [1, 2, 3, 5, 10] {
            if num + offset <= 3000 {
                choices.push(serenity::AutocompleteChoice::new(
                    format!("Comic #{}", num + offset),
                    (num + offset).to_string(),
                ));
            }
            if num >= offset && num - offset > 0 {
                choices.push(serenity::AutocompleteChoice::new(
                    format!("Comic #{}", num - offset),
                    (num - offset).to_string(),
                ));
            }
        }
    } else {
        // If partial text doesn't parse as number, suggest some popular comics based on keywords
        let search_term = partial.to_lowercase();
        let popular_comics = [
            (353, "python"),
            (927, "standards"),
            (386, "duty calls"),
            (303, "compiling"),
            (1205, "time worth"),
            (1053, "ten thousand"),
            (2347, "dependency"),
            (149, "sandwich"),
            (1579, "tech loops"),
            (1319, "automation"),
        ];

        for (num, keywords) in popular_comics.iter() {
            if keywords.contains(&search_term) {
                choices.push(serenity::AutocompleteChoice::new(
                    format!("Comic #{}", num),
                    num.to_string(),
                ));
            }
        }
    }

    // Limit to 25 choices (Discord's limit)
    let final_choices: Vec<serenity::AutocompleteChoice> = choices.into_iter().take(25).collect();
    serenity::CreateAutocompleteResponse::new().set_choices(final_choices)
}

/// Fetch an xkcd comic
#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Fetch an xkcd comic. Leave empty for latest comic or provide a number."
    )
)]
pub async fn xkcd(
    ctx: Context<'_>,
    #[description = "Comic number (leave empty for latest)"]
    #[autocomplete = "comic_autocomplete"]
    comic_number: Option<u32>,
) -> Result<(), Error> {
    // Get the latest comic info to know the range
    let latest_response = reqwest::get("https://xkcd.com/info.0.json")
        .await?
        .json::<Value>()
        .await?;

    let latest_num = latest_response["num"].as_u64().unwrap_or(1) as u32;

    // Determine which comic to fetch
    let target_num = if let Some(num) = comic_number {
        if num == 0 || num > latest_num {
            latest_num // Send latest comic if invalid number
        } else {
            num
        }
    } else {
        latest_num
    };

    let explain_url = format!("https://explainxkcd.com/{}", target_num);
    let xkcd_url = format!("https://xkcd.com/{}", target_num);

    // Create button component for explanation
    let button = CreateButton::new_link(explain_url).label("Explain xkcd");

    let components = vec![CreateActionRow::Buttons(vec![button].into())];

    // Send the xkcd link as plain text with explanation button
    ctx.send(
        poise::CreateReply::default()
            .content(xkcd_url)
            .components(components),
    )
    .await?;
    Ok(())
}
