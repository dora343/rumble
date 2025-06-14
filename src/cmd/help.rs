use crate::cmd::Context;
use crate::cmd::Error;

/// Displays the author's account creation date
#[poise::command(prefix_command, aliases("h"))]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    // obtain the author as user if not specified
    let response: String = "This feature is disabled, for now.".into();
    println!("{}", response);
    ctx.say(response).await?;
    Ok(())
}
