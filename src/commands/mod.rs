mod about;
mod advice;
mod dad_joke;
mod fact;
mod fortune;
mod generate_meme;
mod let_me_google_that;
mod pickup_line;
mod ping;
mod register;
mod say;
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
        fortune::fortune(),
        generate_meme::generate_meme(),
        let_me_google_that::let_me_google_that(),
        pickup_line::pickup_line(),
        ping::ping(),
        register::register(),
        say::say(),
        user_info::user_info(),
        xkcd::xkcd(),
    ]
}
