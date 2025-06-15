pub mod age;
pub mod recursion_test;
pub mod help;
pub mod gamble;
pub mod register;

use crate::Data;
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
