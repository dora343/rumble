use crate::serenity;
pub mod hello;
use hello::respond_hello;

pub mod ping_dora;
use ping_dora::respond_ping_dora;

use serenity::model::channel::Message;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn message_handler(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    if msg.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    match &msg.content {
        _ if msg.content.contains("hello") => respond_hello(ctx, msg).await,
        _ if msg.content.eq("dora") => respond_ping_dora(ctx, msg).await,
        _ => Ok(()),
    }
}
