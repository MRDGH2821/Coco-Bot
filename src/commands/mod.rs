mod user_info;

use crate::Data;
use crate::Error;
use poise::Command;

pub fn all_commands() -> Vec<Command<Data, Error>> {
    vec![user_info::user_info()]
}
