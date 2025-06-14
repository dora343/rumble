use crate::event_handler::message::Error;
use crate::event_handler::message::Message;
use crate::serenity;

pub async fn respond_ping_dora(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    // hard code directly, no reason to make this call flexible
    let reaction: String = "<@400941378395439104>".into();
    println!("{}", reaction);
    msg.channel_id.say(ctx, reaction).await?;
    Ok(())
}
