use poise::serenity_prelude as serenity;
use serenity::model::channel::Message;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn respond_hello(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    
    let reaction = format!("Hello mentioned!");
    print!("{}", reaction);
    msg.reply_ping(ctx, reaction).await?;
    Ok(())
}
