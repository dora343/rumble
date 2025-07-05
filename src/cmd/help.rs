use serenity::all::MessageBuilder;

use crate::cmd::Context;
use crate::cmd::Error;

/// Displays the author's account creation date
#[poise::command(prefix_command, slash_command, aliases("h"))]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    // obtain the author as user if not specified
    let response: String = MessageBuilder::new()
        .push_line("# Available Commands:")
        // .push_line("```rust")
        .push_line("")
        .push_line("### Help Command")
        .push_line("  Command: `.help`")
        .push_line("    Alias: `.h`")
        .push_line("### Register your account")
        .push_line("  Command: `.g register`")
        .push_line("    Alias: `.g reg`")
        .push_line("### Gamble with your tokens")
        .push_line("  Command: `.gamble <your bet>`")
        .push_line("    Alias: `.g <your bet>`")
        .push_line("### Toggle auto revive")
        .push_line("  Command: `.gamble autorevive`")
        .push_line("    Alias: `.g autorevive`")
        .push_line("### Display statistics")
        .push_line("  Command: `.gamble statistics`")
        .push_line("    Alias: `.g stat`")
        .push_line("### Use all your tokens as bet")
        .push_line("  Command: `.gamble allin`")
        .push_line("    Alias: `.g all`")
        .push_line("### Display leaderboard")
        .push_line("  Command: `.gamble leaderboard`")
        .push_line("    Alias: `.g lb`")
        .push_line("### Revive")
        .push_line("  Command: `.gamble revive`")
        .push_line("    Alias: `.g rev`")
        // .push_line("```")
        .build();
    // println!("{}", response);
    ctx.say(response).await?;
    Ok(())
}
