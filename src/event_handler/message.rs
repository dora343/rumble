use crate::{Data, serenity};
pub mod hello;
use hello::respond_hello;

pub mod ping_dora;
use ping_dora::respond_ping_dora;

pub mod get_pinged;
use get_pinged::respond_get_pinged;

pub mod twitter_link;
use twitter_link::respond_twitter_link;

use regex::Regex;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn message_handler(
    ctx: &serenity::Context,
    _data: &Data,
    msg: &mut Message,
) -> Result<(), Error> {
    if msg.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    let ping_bot = MessageBuilder::new()
        .mention(&ctx.cache.current_user().id)
        .build();
        
    let twitter_x_regex = Regex::new(r"https:\/\/(x|twitter)\.com\/[A-Za-z0-9_]{1,15}\/status\/([0-9]+)").unwrap();

    match msg {
        msg if msg.content.eq("dora") => respond_ping_dora(ctx, msg).await,
        msg if msg.content.contains("hello") => respond_hello(ctx, msg).await,
        msg if msg.content.contains(&ping_bot) => respond_get_pinged(ctx, msg).await,
        msg if twitter_x_regex.is_match(&msg.content) => respond_twitter_link(ctx, msg).await,
        _ => Ok(()),
    }
}
