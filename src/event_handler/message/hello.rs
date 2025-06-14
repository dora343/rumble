use crate::event_handler::message::Error;
use crate::event_handler::message::Message;
use crate::serenity;

pub async fn respond_hello(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    let reaction = format!("Hello mentioned!");
    println!("{}", reaction);
    msg.reply_ping(ctx, reaction).await?;
    Ok(())
}
