use std::cmp::min;

use chrono::{Days, Local};
use serenity::all::MessageBuilder;

use crate::cmd::{Context, Error};
use crate::minigame::gamble::{self, MAX_LOGIN_BUFF_ROUNDS, MIN_LOGIN_BUFF_ROUNDS};

pub async fn handle_login(ctx: Context<'_>) -> Result<String, Error> {
    let user_id = ctx.author().id;

    let res: Option<gamble::DailyLogin> = sqlx::query_as(
        r#"
        select 
            id,
            login_combo,
            buff_remaining_rounds,
            last_login
        from gamble.daily_login
        where id = $1;
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&ctx.data().dbpool)
    .await?;

    match res {
        None => Ok(MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.g register` to get registered.")
            .build()),
        Some(res) => {
            let today = Local::now().date_naive();
            let yesterday = Local::now()
                .checked_sub_days(Days::new(1))
                .unwrap()
                .date_naive();
            // 4 possible situation:
            //      1: last login day > today (time traveller?)
            //      2: last login day == today (re-login)
            //      3: last login day < yesterday (combo break)
            //      4: last login day == yesterday (keep combo)
            match res {
                _ if res.last_login.date_naive() == today => {
                    let msg = MessageBuilder::new()
                        .push("You cannot login twice a day.\n")
                        .build();
                    Ok(msg)
                }

                _ if res.last_login.date_naive() == yesterday => {
                    // combo + 1
                    let buff_rounds = min(res.login_combo + MIN_LOGIN_BUFF_ROUNDS, MAX_LOGIN_BUFF_ROUNDS);
                    sqlx::query(
                        r#"
                        update gamble.daily_login
                        set 
                            login_combo = $1,
                            buff_remaining_rounds = $2,
                            last_login = $3
                        where id = $4
                        "#,
                    )
                    .bind(res.login_combo + 1)
                    .bind(buff_rounds)
                    .bind(Local::now())
                    .bind(res.id)
                    .execute(&ctx.data().dbpool)
                    .await?;

                    let msg = MessageBuilder::new()
                        .push("### Login success!\n")
                        .push(format!("Current Daily Login Streak: {}\n", res.login_combo + 1))
                        .push(format!("Login Bonus remaining round(s): {}", buff_rounds))
                        .build();
                    Ok(msg)
                }
                _ => {
                    // reset combo
                    sqlx::query(
                        r#"
                        update gamble.daily_login
                        set 
                            login_combo = $1,
                            buff_remaining_rounds = $2,
                            last_login = $3
                        where id = $4
                        "#,
                    )
                    .bind(1)
                    .bind(MIN_LOGIN_BUFF_ROUNDS)
                    .bind(Local::now())
                    .bind(res.id)
                    .execute(&ctx.data().dbpool)
                    .await?;

                    let msg = MessageBuilder::new()
                        .push("### Login success!\n")
                        .push(format!("Current Daily Login Streak: {}\n", 1))
                        .push(format!("Login Bonus Remaining round(s): {}", MIN_LOGIN_BUFF_ROUNDS))
                        .build();
                    Ok(msg)
                }
            }
        }
    }
}
