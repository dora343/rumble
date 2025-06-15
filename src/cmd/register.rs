use crate::cmd::Context;
use crate::cmd::Error;
use crate::minigame::gamble::handle_register::handle_register;

/// Displays the author's account creation date
#[poise::command(prefix_command, aliases("reg"), help_text_fn = "help_register")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    let msg = handle_register(ctx.data(), ctx.author().id).await?;
    ctx.reply(msg).await?;
    Ok(())
}

fn help_register() -> String {
    let prefix = std::env::var("COMMAND_PREFIX").unwrap();

    serenity::utils::MessageBuilder::new()
        .push("\nUsage: `")
        .push(&prefix)
        .push("register`\n")
        .push("Alias: `")
        .push(&prefix)
        .push("reg`")
        .build()
}
