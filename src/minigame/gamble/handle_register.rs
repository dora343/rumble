use crate::cmd::{Context, Error};
use crate::minigame::gamble::{
    DEFAULT_AUTO_REVIVE, DEFAULT_CRIT_MUL, DEFAULT_CRIT_RATE, DEFAULT_RATE, DEFAULT_REVIVE_TOKENS,
    DEFAULT_TOKENS,
};

pub async fn handle_register(ctx: Context<'_>) -> Result<String, Error> {
    // lookup user_id
    // early return if found

    let user_id = ctx.author().id;

    let res = sqlx::query(
        r#"
        select * from gamble.users
        where id = $1
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&ctx.data().dbpool)
    .await?;

    match res {
        Some(_) => Ok(String::from("You are already registered.")),
        None => {
            let username = user_id.to_user(ctx).await?.name;

            println!("Inserting new player {} into gamble.users", user_id);
            let res = sqlx::query(
                r#"
                insert into gamble.users (id, name, tokens, rate, crit_rate, crit_mul, revive_tokens, auto_revive)
                values ($1, $2, $3, $4, $5, $6, $7)
                "#
            )
            .bind(user_id.get() as i64)
            .bind(username)
            .bind(DEFAULT_TOKENS)
            .bind(DEFAULT_RATE)
            .bind(DEFAULT_CRIT_RATE)
            .bind(DEFAULT_CRIT_MUL)
            .bind(DEFAULT_REVIVE_TOKENS)
            .bind(DEFAULT_AUTO_REVIVE)
            .execute(&ctx.data().dbpool)
            .await?;

            println!("Affected rows: {}", res.rows_affected());

            println!("Inserting new player {} into gamble.user_stat", user_id);
            let res = sqlx::query(
                r#"
                insert into gamble.user_stat (id, max_tokens)
                values ($1, $2)
                "#,
            )
            .bind(user_id.get() as i64)
            .bind(DEFAULT_TOKENS)
            .execute(&ctx.data().dbpool)
            .await?;

            println!("Affected rows: {}", res.rows_affected());

            Ok(String::from("Successfully registered."))
        }
    }
}
