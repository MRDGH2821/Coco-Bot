use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Create a code block for Discord
fn code_block(s: &str) -> String {
    format!("```\n{}\n```", s)
}

/// Autocomplete function for cowsay characters
async fn character_autocomplete<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> serenity::CreateAutocompleteResponse<'a> {
    let characters = vec![
        ("beavis", "Haha hehe he ha he he ha ha"),
        (
            "bong",
            "Sometimes cows need to take a load off and have a conversation",
        ),
        ("bud-frogs", "Talk to some frogs"),
        ("bunny", "Aww, a cute little bunny :3"),
        ("cheese", "Talking cheese. Normal enough..."),
        ("cower", "Sometimes cows are scared of you"),
        ("daemon", "he's gon' poke u"),
        ("default", "The default cow that everybody knows and loves"),
        ("doge", "Much cow. Very talk. Wow"),
        ("dragon-and-cow", "Sometimes cows are terrorized by dragons"),
        (
            "dragon",
            "Sometimes the dragons aren't terrorizing anything at all",
        ),
        ("elephant-in-snake", "I guess a snake ate an elephant"),
        ("elephant", "The non-eaten elephant"),
        ("eyes", "Very large eyes"),
        (
            "flaming-sheep",
            "Sometimes sheep burn (probably from the dragon)",
        ),
        ("ghostbusters", "Who ya gonna call!? GhOsTbUsTeRs!!"),
        ("goat", "A nice little goat"),
        ("hedgehog", "Everybody loves hedgehogs"),
        ("hellokitty", "Hello, *meow*"),
        ("kiss", "Aww look they're in love <3"),
        ("kitty", "MeOw"),
        ("koala", "Oh I just want to squish it!"),
        ("kosh", "A very square kosh"),
        ("luke-koala", "\"Koala, I am your father\""),
        (
            "mech-and-cow",
            "That cow is finally about to get an upgrade",
        ),
        (
            "meow",
            "Wow thats a deep sounding meow.. I wonder what size that cat i.....",
        ),
        ("milk", "Don't cry over me"),
        ("moofasa", "A king is born!"),
        ("moose", "One of the signature animals in Canada"),
        ("mutilated", "A torn apart cow"),
        ("ren", "Quote ren"),
        (
            "satanic",
            "The ruler of the cow underworld, has allowed you to quote him",
        ),
        ("sheep", "The non-flaming sheep"),
        ("skeleton", "The skeleton of that poor cow"),
        ("small", "A little baby cow"),
        ("squirrel", "A little squirrel that will saw what you want"),
        ("stegosaurus", "A talking stego"),
        ("stimpy", "Let stimpy talk"),
        ("supermilker", "Supermilk that poor poor cow"),
        (
            "surgery",
            "That cow is probably going to get revenge... Uh oh..",
        ),
        (
            "telebears",
            "Woot! Finally that cow is getting some action!",
        ),
        ("turkey", "*Gobble Gobble*"),
        (
            "turtle",
            "Carry around your words in your own portable home",
        ),
        ("tux", "Everybody's favourite linux penguin!"),
    ];

    let mut choices = Vec::new();
    let search_term = partial.to_lowercase();

    if partial.is_empty() {
        // If partial is empty, return 25 random characters
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        let mut random_chars: Vec<_> = characters.clone();
        random_chars.shuffle(&mut rng);

        for (name, description) in random_chars.iter().take(25) {
            choices.push(serenity::AutocompleteChoice::new(
                format!("{} - {}", name, description),
                name.to_string(),
            ));
        }
    } else {
        // Filter characters based on the partial input
        for (name, description) in characters.iter() {
            if name.contains(&search_term) || description.to_lowercase().contains(&search_term) {
                choices.push(serenity::AutocompleteChoice::new(
                    format!("{} - {}", name, description),
                    name.to_string(),
                ));
            }
        }
    }

    // Limit to 25 choices (Discord's limit)
    let final_choices: Vec<serenity::AutocompleteChoice> = choices.into_iter().take(25).collect();
    serenity::CreateAutocompleteResponse::new().set_choices(final_choices)
}

/// Call the cowsay API to generate ASCII art
async fn call_cowsay_api(message: &str, cow_type: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = "https://easyapis.honeybeeks.net/api/cowsay";

    let response = client
        .get(url)
        .query(&[("text", message), ("type", cow_type)])
        .send()
        .await?;

    let response_text = response.text().await?;

    // The API returns plain text, not JSON
    Ok(code_block(&response_text))
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Say a custom message with an ASCII character.")
)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message to say"] message: String,
    #[description = "Which character?"]
    #[autocomplete = "character_autocomplete"]
    character: Option<String>,
) -> Result<(), Error> {
    let character_str = character.unwrap_or_else(|| "default".to_string());

    let message_text = message.trim().to_string();

    let response = call_cowsay_api(&message_text, &character_str).await?;

    if response.len() > 2000 {
        let too_long_msg = call_cowsay_api("That message was too long.", "default").await?;
        ctx.say(too_long_msg).await?;
    } else {
        ctx.say(response).await?;
    }

    Ok(())
}
