use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays the author's account creation date
#[poise::command(prefix_command)]
pub async fn age(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // obtain the author as user if not specified
    let u = ctx.author();
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    println!("{}", response);
    ctx.say(response).await?;
    Ok(())
}
