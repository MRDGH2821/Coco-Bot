mod advice;
mod ping;
mod register;
mod user_info;

use crate::Data;
use crate::Error;
use poise::Command;

pub fn all_commands() -> Vec<Command<Data, Error>> {
    vec![
        advice::advice(),
        ping::ping(),
        register::register(),
        user_info::user_info(),
    ]
}
