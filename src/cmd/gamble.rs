use crate::cmd::Context;
use crate::cmd::Error;
use crate::minigame::gamble::handle_gamble::handle_gamble;

/// Displays the author's account creation date
#[poise::command(prefix_command, aliases("g"), help_text_fn = "help_gamble")]
pub async fn gamble(ctx: Context<'_>, bet: String) -> Result<(), Error> {
    let msg = handle_gamble(ctx.data(), ctx.author().id, bet).await?;
    ctx.reply(msg).await?;
    Ok(())
}

fn help_gamble() -> String {
    let prefix = std::env::var("COMMAND_PREFIX").unwrap();

    serenity::utils::MessageBuilder::new()
        .push("\nUsage: `")
        .push(&prefix)
        .push("gamble <tokens>`\n")
        .push("Alias: `")
        .push(&prefix)
        .push("g <tokens>`")
        .build()
}
