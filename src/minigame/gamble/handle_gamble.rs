use serenity::all::UserId;

use serenity::utils::MessageBuilder;

use crate::Data;
use crate::minigame::gamble::handle_revive::handle_revive;
use crate::minigame::gamble::{self, core};

pub async fn handle_gamble(
    data: &Data,
    user_id: UserId,
    bet: String,
) -> Result<String, sqlx::Error> {
    let res: Option<gamble::User> = sqlx::query_as(
        r#"
        select 
            u.id,
            u.tokens,
            s.play_count,
            u.rate,
            u.crit_rate,
            u.crit_mul,
            u.revive_tokens,
            u.auto_revive,
            s.success_count,
            s.fail_count,
            s.revive_count,
            s.successive_success,
            s.successive_fail,
            s.max_tokens,
            s.max_bet, 
            s.max_success_bet,
            s.max_fail_bet,
            s.max_successive_success,
            s.max_successive_fail
        from gamble.users u
        left join gamble.user_stat s
        on u.id = s.id
        where u.id = $1;
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&data.dbpool)
    .await?;

    if let None = res {
        return Ok(MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.register` to get registered.")
            .build());
    }

    // TODO: Initialize stat table if not exists

    match bet.trim().parse::<i64>() {
        Ok(bet) => {
            // lookup user_tokens
            // early return if not found, or bet > user_tokens
            let user = res.unwrap();
            let user_tokens = user.tokens;

            if bet > user_tokens {
                return Ok(MessageBuilder::new()
                    .push("You do not have enough tokens.\n")
                    .push(format!("You have {} tokens.", user_tokens))
                    .build());
            }

            match bet {
                ..0 => {
                    // Negative Bet
                    Ok(MessageBuilder::new()
                        .push("You cannot place negative bet.\n")
                        .push(format!("You have {} tokens.", user_tokens))
                        .build())
                }

                0 => {
                    // Zero Bet
                    Ok(MessageBuilder::new()
                        .push("You cannot place zero bet.\n")
                        .push(format!("You have {} tokens.", user_tokens))
                        .build())
                }

                _ => {
                    let result = core::gamble(user.clone(), bet)
                        .update_user(&data.dbpool)
                        .await?
                        .insert_record(&data.dbpool)
                        .await?
                        .update_user_stat(&data.dbpool)
                        .await?;

                    Ok(match result.success {
                        true => MessageBuilder::new()
                            .push("Success! ")
                            .push_bold(format!("{} tokens ", result.differential))
                            .push("have been added to your profile.\n")
                            .push(format!("You have {} tokens.", result.tokens_after))
                            .build(),
                        false => match result.tokens_after == 0 {
                            true => {
                                let fail_msg = MessageBuilder::new()
                                    .push("Unlucky! ")
                                    .push_bold(format!("{} tokens ", result.differential.abs()))
                                    .push("have been taken from your profile.\n")
                                    .build();

                                match user.auto_revive {
                                    true => {
                                        let revive = handle_revive(data, user_id).await?;

                                        MessageBuilder::new()
                                            .push(fail_msg)
                                            .push_italic_line("Auto Revive activated.")
                                            .push(revive)
                                            .build()
                                    }
                                    false => MessageBuilder::new()
                                        .push(fail_msg)
                                        .push("Use `.revive` to start again.")
                                        .build(),
                                }
                            }
                            false => MessageBuilder::new()
                                .push("Unlucky! ")
                                .push_bold(format!("{} tokens ", result.differential.abs()))
                                .push("have been taken from your profile.\n")
                                .push(format!("You have {} tokens.", result.tokens_after))
                                .build(),
                        },
                    })
                }
            }
        }
        Err(_) => Ok(String::from("err: cant parse")),
    }
}

#[derive(sqlx::FromRow, Debug)]
struct AutoReviveInfo {
    id: i64,
    auto_revive: bool,
}

pub async fn handle_autorevive(data: &Data, user_id: UserId) -> Result<String, sqlx::Error> {
    let res: Option<AutoReviveInfo> = sqlx::query_as(
        r#"
        select 
            id,
            auto_revive
        from gamble.users
        where id = $1;
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&data.dbpool)
    .await?;

    match res {
        Some(auto_revive_info) => {
            println!("Updating user {:?} in gamble.users", auto_revive_info.id);
            let res = sqlx::query(
                r#"
                update gamble.users
                set 
                    auto_revive = $1
                where id = $2
                "#,
            )
            .bind(!auto_revive_info.auto_revive)
            .bind(auto_revive_info.id)
            .execute(&data.dbpool)
            .await?;
            println!("Affected rows: {}", res.rows_affected());

            let on_off = match auto_revive_info.auto_revive {
                true => "off",
                false => "on",
            };

            Ok(MessageBuilder::new()
                .push(format!("Auto revive switched {on_off}."))
                .build())
        }
        None => Ok(MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.register` to get registered.")
            .build()),
    }
}
