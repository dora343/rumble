use crate::cmd::Context;
use crate::cmd::Error;

/// Displays the author's account creation date
#[poise::command(prefix_command, aliases("rrrrr"))]
pub async fn recursion_test(ctx: Context<'_>) -> Result<(), Error> {
    // obtain the author as user if not specified
    let response: String = ".recursion_test".into();
    println!("{}", response);
    ctx.say(response).await?;
    Ok(())
}
