use crate::cmd::Context;
use crate::cmd::Error;

/// This is a wip game.
#[poise::command(slash_command)]
pub async fn astral(ctx: Context<'_>) -> Result<(), Error> {
    // let msg = handle_revive(ctx.data(), ctx.author().id).await?;
    let msg = String::from("astral");
    ctx.reply(msg).await?;
    Ok(())
}
