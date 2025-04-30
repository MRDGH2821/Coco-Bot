mod age;

use crate::Data;
use crate::Error;
use poise::Command;

pub fn all_commands() -> Vec<Command<Data, Error>> {
    vec![age::age()]
}
