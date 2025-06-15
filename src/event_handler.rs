use poise::serenity_prelude as serenity;
pub mod message;

use crate::{Data, event_handler::message::message_handler};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            message_handler(ctx, data, new_message).await?;
        }
        _ => {}
    }
    Ok(())
}
