use crate::cmd::Context;
use crate::cmd::Error;

/// Displays the author's account creation date
#[poise::command(prefix_command, aliases("rev"), help_text_fn = "help_revive")]
pub async fn revive(ctx: Context<'_>) -> Result<(), Error> {
    // let msg = handle_revive(ctx.data(), ctx.author().id).await?;
    let msg = String::from("Use `.g revive` to revive.");
    ctx.reply(msg).await?;
    Ok(())
}

fn help_revive() -> String {
    let prefix = std::env::var("COMMAND_PREFIX").unwrap();

    serenity::utils::MessageBuilder::new()
        .push("\nUsage: `")
        .push(&prefix)
        .push("revive`\n")
        .push("Alias: `")
        .push(&prefix)
        .push("rev`")
        .build()
}
