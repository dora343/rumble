pub mod age;
pub mod gamble;
pub mod help;
pub mod recursion_test;
pub mod register;

use crate::Data;
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
