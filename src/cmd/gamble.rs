use crate::cmd::Context;
use crate::cmd::Error;
use crate::minigame::gamble::handle_gamble::handle_allin;
use crate::minigame::gamble::handle_gamble::handle_autorevive;
use crate::minigame::gamble::handle_gamble::handle_gamble;
use crate::minigame::gamble::handle_gamble::handle_leaderboard;
use crate::minigame::gamble::handle_gamble::handle_statistics;

/// Displays the author's account creation date
#[poise::command(
    prefix_command,
    subcommands("autorevive", "leaderboard", "statistics", "allin"),
    aliases("g"),
    help_text_fn = "help_gamble"
)]
pub async fn gamble(ctx: Context<'_>, bet: String) -> Result<(), Error> {
    let msg = handle_gamble(ctx, bet).await?;
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

#[poise::command(prefix_command)]
pub async fn autorevive(ctx: Context<'_>) -> Result<(), Error> {
    let msg = handle_autorevive(ctx.data(), ctx.author().id).await?;
    ctx.reply(msg).await?;
    Ok(())
}

#[poise::command(prefix_command, aliases("lb"))]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let msg = handle_leaderboard(ctx).await?;
    ctx.reply(msg).await?;
    Ok(())
}

#[poise::command(prefix_command, aliases("stat"))]
pub async fn statistics(ctx: Context<'_>) -> Result<(), Error> {
    let msg = handle_statistics(ctx).await?;
    ctx.reply(msg).await?;
    Ok(())
}

#[poise::command(prefix_command, aliases("all"))]
pub async fn allin(ctx: Context<'_>) -> Result<(), Error> {
    let msg = handle_allin(ctx).await?;
    ctx.reply(msg).await?;
    Ok(())
}
