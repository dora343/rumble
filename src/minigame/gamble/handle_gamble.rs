use serenity::all::UserId;

use serenity::utils::MessageBuilder;

pub async fn handle_gamble(user_id: UserId, bet: String) -> String {
    // Err(GambleError::UserNotRegistered)
    // Ok("".into())
    
    match bet.trim().parse::<i128>() {
        Ok(bet) => {
            // lookup user_tokens
            // early return if not found, or bet > user_tokens
            
            match bet {
                ..0 => {
                    // Negative Bet
                    MessageBuilder::new()
                        .push("You cannot place negative bet.\n")
                        .push(format!("You have {} tokens.", 0))
                        .build()
                }
                
                0 => {
                    // Zero Bet
                    String::from("")
                }
                
                _ => {
                    // Valid bet
                    // do random
                    // record stats to profile
                    // handle items here if applicable
                    
                    String::from("")
                }
            }
        },
        Err(_) => {
            String::from("")
        },
    }
}