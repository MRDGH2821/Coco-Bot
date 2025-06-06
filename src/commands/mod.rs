mod about;
mod advice;
mod meme_generator;
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
        meme_generator::meme_generator(),
        ping::ping(),
        register::register(),
        user_info::user_info(),
        xkcd::xkcd(),
    ]
}
