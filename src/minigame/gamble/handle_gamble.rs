use serenity::all::UserId;

use serenity::utils::MessageBuilder;
use unicode_width::UnicodeWidthStr;

use crate::Data;
use crate::cmd::Context;
use crate::minigame::gamble::handle_revive::handle_revive;
use crate::minigame::gamble::{
    self, AutoReviveInfo, LeaderboardProfile, Statistics, UserTokens, core,
};

pub async fn handle_gamble(ctx: Context<'_>, bet: String) -> Result<String, crate::cmd::Error> {
    let user_id = ctx.author().id;

    let res: Option<gamble::User> = sqlx::query_as(
        r#"
        select 
            u.id,
            u.tokens,
            u.tp,
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
            s.max_successive_fail,
            l.buff_remaining_rounds,
            l.last_login
        from gamble.users u
        join gamble.user_stat s
        on u.id = s.id
        join gamble.daily_login l
        on u.id = l.id
        where u.id = $1;
        "#,
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&ctx.data().dbpool)
    .await?;

    if let None = res {
        return Ok(MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.g register` to get registered.")
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
                        .update_user(&ctx.data().dbpool)
                        .await?
                        .insert_record(&ctx.data().dbpool)
                        .await?
                        .update_user_stat(&ctx.data().dbpool)
                        .await?
                        .update_daily_login(&ctx.data().dbpool)
                        .await?;

                    let mut msg = match result.success {
                        true => MessageBuilder::new()
                            .push("Success! ")
                            .push_bold(format!("{} tokens ", result.differential))
                            .push("have been added to your profile.\n")
                            .push(format!("You have {} tokens.", result.tokens_after))
                            .build(),
                        false => MessageBuilder::new()
                            .push("Unlucky! ")
                            .push_bold(format!("{} tokens ", result.differential.abs()))
                            .push("have been taken from your profile.\n")
                            .push(format!("You have {} tokens.", result.tokens_after))
                            .build(),
                    };

                    if result.buff_remaining_rounds > 0 {
                        msg = MessageBuilder::new()
                            .push_line(msg)
                            .push_italic(format!(
                                "Login Bonus remaining round(s): {}",
                                result.buff_remaining_rounds
                            ))
                            .build();
                    }

                    if result.tokens_after == 0 {
                        match user.auto_revive {
                            true => {
                                let revive = handle_revive(ctx, user_id).await?;

                                msg = MessageBuilder::new()
                                    .push_line(msg)
                                    .push_italic_line("Auto Revive activated.")
                                    .push(revive)
                                    .build();
                            }
                            false => {
                                msg = MessageBuilder::new()
                                    .push_line(msg)
                                    .push("Use `.g revive` to start again.")
                                    .build();
                            }
                        }
                    }

                    if result.tp != user.tp {
                        msg = MessageBuilder::new()
                            .push_line("Success!")
                            .push_italic_line("Auto Compress activated.")
                            .push_line(format!("You have **{}** TP. _(+{})_", result.tp, result.tp - user.tp))
                            .push_line(format!("You have {} Tokens.", result.tokens_after))
                            .build();
                    }

                    Ok(msg)
                }
            }
        }
        Err(_) => Ok(String::from("err: cant parse")),
    }
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
            sqlx::query(
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
            .push("Use `.g register` to get registered.")
            .build()),
    }
}

pub async fn handle_leaderboard(ctx: Context<'_>) -> Result<String, crate::cmd::Error> {
    let res: Vec<LeaderboardProfile> = sqlx::query_as(
        r#"
        select 
            id,
            name,
            tp,
            tokens
        from gamble.users
        order by tp desc, tokens desc;
        "#,
    )
    .fetch_all(&ctx.data().dbpool)
    .await?;

    if res.is_empty() {
        return Ok("No one has registered.".into());
    }

    let max_name_len = std::cmp::max(6, res.iter().map(|x| x.name.len()).max().unwrap());
    
    let max_tp_len = std::cmp::max(2, res.iter().map(|x| x.tp.checked_ilog10().unwrap_or(0) + 1).max().unwrap());
    
    let mut msg: String = String::from("Rank\tPlayer");

    let title_indent_before_tokens = 2 + (max_name_len - 6);
    for _ in 0..title_indent_before_tokens {
        msg.push(' ');
    }
    msg.push_str("TP");
    for _ in 0..(3 + max_tp_len - 2) {
        msg.push(' ');
    }
    msg.push_str("Tokens\n");

    for (index, player_profile) in res.iter().enumerate() {
        if player_profile.name == "DEFAULT_PLACE_HOLDER" {
            let user_id = UserId::from(player_profile.id as u64);

            // huge bottleneck here
            let username = user_id.to_user(ctx).await?.name;

            let _ = sqlx::query(
                r#"
                update gamble.users
                set 
                    name = $1
                where id = $2
                "#,
            )
            .bind(&username)
            .bind(player_profile.id)
            .execute(&ctx.data().dbpool)
            .await?;

            let indent_before_tokens: usize = 2 + (max_name_len - username.width_cjk());

            let mut indent1 = String::from("");

            for _ in 0..indent_before_tokens {
                indent1.push(' ');
            }
            
            let indent_before_tp: u32 = 1 + (max_tp_len - player_profile.tp.checked_ilog10().unwrap_or(0) + 1);
            let mut indent2 = String::from("");
            for _ in 0..indent_before_tp {
                indent2.push(' ');
            }

            msg.push_str(
                &MessageBuilder::new()
                    .push_line(format!(
                        "{}\t   {}{}{}{}{}",
                        index + 1,
                        username,
                        indent1,
                        player_profile.tp,
                        indent2,
                        player_profile.tokens
                    ))
                    .build(),
            );
        } else {
            let indent_before_tokens: usize = 2 + (max_name_len - player_profile.name.width_cjk());

            let mut indent1 = String::from("");

            for _ in 0..indent_before_tokens {
                indent1.push(' ');
            }
            
            let indent_before_tp: u32 = 1 + (max_tp_len - player_profile.tp.checked_ilog10().unwrap_or(0) + 1);
            let mut indent2 = String::from("");
            for _ in 0..indent_before_tp {
                indent2.push(' ');
            }

            msg.push_str(
                &MessageBuilder::new()
                    .push_line(format!(
                        "{}\t   {}{}{}{}{}",
                        index + 1,
                        player_profile.name,
                        indent1,
                        player_profile.tp,
                        indent2,
                        player_profile.tokens
                    ))
                    .build(),
            );
        }
    }

    Ok(MessageBuilder::new()
        .push_line("# Leaderboard")
        .push_codeblock(msg, Some("rust"))
        .build())
}

pub async fn handle_statistics(ctx: Context<'_>) -> Result<String, crate::cmd::Error> {
    let res: Option<Statistics> = sqlx::query_as(
        r#"
        select 
            u.tokens,
            u.tp,
            u.auto_revive,
            s.play_count,
            s.success_count,
            s.fail_count,
            s.revive_count,
            s.max_tokens,
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
    .bind(ctx.author().id.get() as i64)
    .fetch_optional(&ctx.data().dbpool)
    .await?;

    match res {
        Some(stats) => Ok(MessageBuilder::new()
            .push_line("# Statistics")
            .push_line("```rust")
            .push_line(format!("                      Tokens: {}", stats.tokens))
            .push_line(format!("                          TP: {}", stats.tp))
            .push_line(format!(
                "                 Auto Revive: {}",
                stats.auto_revive
            ))
            .push_line(format!(
                "                  Play Count: {}",
                stats.play_count
            ))
            .push_line(format!(
                "               Total Success: {}",
                stats.success_count
            ))
            .push_line(format!(
                "               Total Failure: {}",
                stats.fail_count
            ))
            .push_line(format!(
                "                Revive Count: {}",
                stats.revive_count
            ))
            .push_line(format!(
                "              Highest Tokens: {}",
                stats.max_tokens
            ))
            .push_line(format!(
                "      Highest Successful Bet: {}",
                stats.max_success_bet
            ))
            .push_line(format!(
                "    Highest Unsuccessful Bet: {}",
                stats.max_fail_bet
            ))
            .push_line(format!(
                " Highest Consecutive Success: {}",
                stats.max_successive_success
            ))
            .push_line(format!(
                " Highest Consecutive Failure: {}",
                stats.max_successive_fail
            ))
            .push_line("```")
            .build()),
        None => Ok(MessageBuilder::new()
            .push("You are not registered.\n")
            .push("Use `.register` to get registered.")
            .build()),
    }
}

pub async fn handle_allin(ctx: Context<'_>) -> Result<String, crate::cmd::Error> {
    let res: UserTokens = sqlx::query_as(
        r#"
        select 
            tokens
        from gamble.users
        where id = $1
        "#,
    )
    .bind(ctx.author().id.get() as i64)
    .fetch_one(&ctx.data().dbpool)
    .await?;

    handle_gamble(ctx, res.0.to_string()).await
}
