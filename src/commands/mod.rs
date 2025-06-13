mod about;
mod advice;
mod dad_joke;
mod fact;
mod generate_meme;
mod let_me_google_that;
mod ping;
mod register;
mod user_info;
mod xkcd;

use crate::Data;
use crate::Error;
use poise::Command;

pub fn all_commands() -> Vec<Command<Data, Error>> {
    vec![
        about::about(),
        advice::advice(),
        dad_joke::dad_joke(),
        fact::fact(),
        generate_meme::generate_meme(),
        let_me_google_that::let_me_google_that(),
        ping::ping(),
        register::register(),
        user_info::user_info(),
        xkcd::xkcd(),
    ]
}
