use crate::cmd::Context;
use crate::cmd::Error;

/// Displays the author's account creation date
#[poise::command(
    prefix_command,
    aliases("reg"),
    owners_only,
    help_text_fn = "help_register"
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // let msg = handle_register(ctx).await?;
    // ctx.reply(msg).await?;
    poise::builtins::register_application_commands_buttons(ctx).await?;
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
