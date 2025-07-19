pub mod age;
pub mod gamble;
pub mod help;
pub mod album;
pub mod register;
pub mod revive;
pub mod astral;

use crate::Data;
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
