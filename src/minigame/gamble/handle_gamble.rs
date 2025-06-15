use serenity::all::UserId;

use serenity::utils::MessageBuilder;

use crate::minigame::gamble;
use crate::Data;

pub async fn handle_gamble(data: &Data, user_id: UserId, bet: String, ) -> Result<String, sqlx::Error> {
    // Err(GambleError::UserNotRegistered)
    // Ok("".into())
    let res: Option<gamble::User> = sqlx::query_as(
        r#"
        select * from gamble.users
        where id = $1
        "#
    )
    .bind(user_id.get() as i64)
    .fetch_optional(&data.dbpool)
    .await?;
        
    if let None = res {
        return Ok(
            MessageBuilder::new()
            .push("You are not registered.\n")
            .push("To register, use `.register`")
            .build()
        )
    }
    
    
    
    match bet.trim().parse::<i64>() {
        Ok(bet) => {
            // lookup user_tokens
            // early return if not found, or bet > user_tokens
            let user_tokens = res.unwrap().tokens;
            
            if bet > user_tokens {
                return Ok(
                        MessageBuilder::new()
                            .push("You do not have enough tokens.\n")
                            .push(format!("You have {} tokens.", user_tokens))
                            .build()
                )
            }
            
            match bet {
                ..0 => {
                    // Negative Bet
                    Ok(
                        MessageBuilder::new()
                            .push("You cannot place negative bet.\n")
                            .push(format!("You have {} tokens.", user_tokens))
                            .build()
                    )
                }
                
                0 => {
                    // Zero Bet
                    Ok(
                        MessageBuilder::new()
                            .push("You cannot place zero bet.\n")
                            .push(format!("You have {} tokens.", user_tokens))
                            .build()
                    )
                }
                
                _ => {
                    // Valid bet
                    // do random
                    // record stats to profile
                    // handle items here if applicable
                    Ok(
                        String::from("wip")
                    )
                }
            }
        },
        Err(_) => {
            Ok(
                String::from("err: cant parse")
            )
        },
    }
}