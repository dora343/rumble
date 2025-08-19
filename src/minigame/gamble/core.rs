use std::cmp::max;

use chrono::Local;
use rand::Rng;

use crate::minigame::gamble::{User, COMPRESS_REWARD, COMPRESS_TRIGGER, DAILY_LOGIN_BUFF_RATE, MAX_RATE, MULTIPLIER_BASE};

#[derive(Debug)]
pub struct GambleResult {
    user_id: i64,
    play_count: i32,
    bet: i64,
    pub differential: i64,
    pub success: bool,
    pub is_crit: bool,
    rate: i16,
    crit_rate: i16,
    tokens_before: i64,
    pub tokens_after: i64,
    success_count: i32,
    fail_count: i32,
    successive_success: i32,
    successive_fail: i32,
    revive_count: i32,
    max_tokens: i64,
    max_bet: i64,
    max_success_bet: i64,
    max_fail_bet: i64,
    max_successive_success: i32,
    max_successive_fail: i32,
    pub buff_remaining_rounds: i32,
    pub tp: i64,
}

impl GambleResult {
    pub async fn update_user(self, dbpool: &sqlx::PgPool) -> Result<GambleResult, sqlx::Error> {
        sqlx::query(
            r#"
            update gamble.users
            set tokens = $1, tp = $2
            where id = $3
            "#,
        )
        .bind(self.tokens_after)
        .bind(self.tp)
        .bind(self.user_id)
        .execute(dbpool)
        .await?;
        Ok(self)
    }

    pub async fn update_user_stat(
        self,
        dbpool: &sqlx::PgPool,
    ) -> Result<GambleResult, sqlx::Error> {
        sqlx::query(
            r#"
            update gamble.user_stat
            set 
                play_count = $1,
                success_count = $2,
                fail_count = $3,
                revive_count = $4,
                successive_success = $5,
                successive_fail = $6,
                max_tokens = $7,
                max_bet = $8,
                max_success_bet = $9,
                max_fail_bet = $10,
                max_successive_success = $11,
                max_successive_fail = $12
            where id = $13
            "#,
        )
        .bind(self.play_count)
        .bind(self.success_count)
        .bind(self.fail_count)
        .bind(self.revive_count)
        .bind(self.successive_success)
        .bind(self.successive_fail)
        .bind(self.max_tokens)
        .bind(self.max_bet)
        .bind(self.max_success_bet)
        .bind(self.max_fail_bet)
        .bind(self.max_successive_success)
        .bind(self.max_successive_fail)
        .bind(self.user_id)
        .execute(dbpool)
        .await?;
        Ok(self)
    }

    pub async fn insert_record(self, dbpool: &sqlx::PgPool) -> Result<GambleResult, sqlx::Error> {
        sqlx::query(
            r#"
            insert into gamble.records 
            (user_id, play_count, bet, success, is_crit, rate, crit_rate, tokens_before, tokens_after)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(self.user_id)
        .bind(self.play_count)
        .bind(self.bet)
        .bind(self.success)
        .bind(self.is_crit)
        .bind(self.rate)
        .bind(self.crit_rate)
        .bind(self.tokens_before)
        .bind(self.tokens_after)
        .execute(dbpool)
        .await?;
        Ok(self)
    }
    
    pub async fn update_daily_login(self, dbpool: &sqlx::PgPool) -> Result<GambleResult, sqlx::Error> {
        sqlx::query(
            r#"
            update gamble.daily_login
            set buff_remaining_rounds = $1
            where id = $2
            "#,
        )
        .bind(self.buff_remaining_rounds)
        .bind(self.user_id)
        .execute(dbpool)
        .await?;
        Ok(self)
    }
}

pub fn gamble(user: User, bet: i64) -> GambleResult {
    let mut rng = rand::rng();

    let crit_check: i16 = rng.random_range(1..(MAX_RATE as i16));

    let gamble_check: i16 = rng.random_range(1..(MAX_RATE as i16));

    let mut rate = user.rate;

    
    if user.buff_remaining_rounds > 0 && user.last_login.date_naive() == Local::now().date_naive() {
        // fixed rate for login bonus
        rate = DAILY_LOGIN_BUFF_RATE;
    }
    
    let crit_success = user.crit_rate >= crit_check;
    let gamble_success = rate >= gamble_check;

    let play_count = user.play_count + 1;

    let multiplier = match crit_success {
        true => match gamble_success {
            true => user.crit_mul as f64 / MULTIPLIER_BASE,
            false => -user.crit_mul as f64 / MULTIPLIER_BASE,
        },
        false => match gamble_success {
            true => 1.0,
            false => -1.0,
        },
    };

    let differential = (bet as f64 * multiplier).round() as i64;
    
    let mut tp = user.tp;

    let tokens_after = if (user.tokens + differential) as i128 >= user.revive_tokens as i128 * COMPRESS_TRIGGER as i128 {
        // auto compress triggered
        tp = tp + COMPRESS_REWARD;
        user.revive_tokens
    } else {
        user.tokens + differential
    };


    let success_count = match gamble_success {
        true => user.success_count + 1,
        false => user.success_count,
    };

    let fail_count = match gamble_success {
        true => user.fail_count,
        false => user.fail_count + 1,
    };

    let successive_success = match gamble_success {
        true => user.successive_success + 1,
        false => 0,
    };

    let successive_fail = match gamble_success {
        true => 0,
        false => user.successive_fail + 1,
    };

    let max_tokens = match tokens_after > user.max_tokens {
        true => tokens_after,
        false => user.max_tokens,
    };

    let max_bet = match bet > user.max_bet {
        true => bet,
        false => user.max_bet,
    };

    let max_success_bet = match gamble_success && bet > user.max_success_bet {
        true => bet,
        false => user.max_success_bet,
    };

    let max_fail_bet = match !gamble_success && bet > user.max_fail_bet {
        true => bet,
        false => user.max_fail_bet,
    };

    let max_successive_success = match successive_success > user.max_successive_success {
        true => user.max_successive_success + 1,
        false => user.max_successive_success,
    };

    let max_successive_fail = match successive_fail > user.max_successive_fail {
        true => user.max_successive_fail + 1,
        false => user.max_successive_fail,
    };
    
    // only consume buff remaining rounds if failed
    let mut buff_remaining_rounds = match gamble_success {
        false => user.buff_remaining_rounds,
        true => max(user.buff_remaining_rounds - 1, 0)
    };
    
    if user.last_login.date_naive() != Local::now().date_naive() {
        buff_remaining_rounds = 0;
    }
    
    GambleResult {
        user_id: user.id,
        success: gamble_success,
        differential,
        bet,
        is_crit: crit_success,
        play_count,
        rate,
        crit_rate: user.crit_rate,
        tokens_before: user.tokens,
        tokens_after,
        tp,
        success_count,
        fail_count,
        successive_success,
        successive_fail,
        revive_count: user.revive_count,
        max_tokens,
        max_bet,
        max_success_bet,
        max_fail_bet,
        max_successive_success,
        max_successive_fail,
        buff_remaining_rounds,
    }
}
