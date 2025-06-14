use crate::event_handler::message::Error;
use crate::event_handler::message::Message;
use crate::serenity;

pub async fn respond_get_pinged(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    if msg.author.id.to_string() == std::env::var("BOT_OWNER_ID").unwrap() {
        let reaction: String = "<a:huggingcat:1383449250246951002>".into();
        println!("{}", reaction);
        msg.reply_ping(ctx, reaction).await?;
        return Ok(());
    }
    
    let reaction: String = "ping乜撚嘢呀木臭".into();
    println!("{}", reaction);
    msg.channel_id.say(ctx, reaction).await?;
    Ok(())
}
