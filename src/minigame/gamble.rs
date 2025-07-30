use chrono::Local;

pub mod core;
pub mod handle_gamble;
pub mod handle_register;
pub mod handle_revive;
pub mod handle_login;

const DEFAULT_TOKENS: i64 = 10000;
const DEFAULT_RATE: i16 = 5000; // 50%
const DEFAULT_CRIT_RATE: i16 = 0; // 0%
const DEFAULT_CRIT_MUL: i32 = 10000; // 100%
const DEFAULT_REVIVE_TOKENS: i32 = 10000; // 100%
const DEFAULT_AUTO_REVIVE: bool = false;

const MAX_RATE: i16 = 10000;
const MULTIPLIER_BASE: f64 = 10000.0;

const DAILY_LOGIN_BUFF_RATE: i16 = 8000; // 80%
const MIN_LOGIN_BUFF_ROUNDS: i32 = 3;
const MAX_LOGIN_BUFF_ROUNDS: i32 = 5;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct User {
    id: i64,
    tokens: i64,
    play_count: i32,
    rate: i16,
    crit_rate: i16,
    crit_mul: i32,
    revive_tokens: i64,
    auto_revive: bool,
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
    buff_remaining_rounds: i32,
    last_login: chrono::DateTime<Local>,
}

#[derive(sqlx::FromRow, Debug)]
struct AutoReviveInfo {
    id: i64,
    auto_revive: bool,
}

#[derive(sqlx::FromRow, Debug)]
struct LeaderboardProfile {
    id: i64,
    name: String,
    tokens: i64,
}

#[derive(sqlx::FromRow, Debug)]
struct Statistics {
    tokens: i64,
    auto_revive: bool,
    play_count: i32,
    success_count: i32,
    fail_count: i32,
    revive_count: i32,
    max_tokens: i64,
    max_success_bet: i64,
    max_fail_bet: i64,
    max_successive_success: i32,
    max_successive_fail: i32,
}

#[derive(sqlx::FromRow, Debug)]
struct UserTokens(i64);

#[derive(sqlx::FromRow, Debug)]
struct DailyLogin {
    id: i64,
    login_combo: i32,
    buff_remaining_rounds: i32,
    last_login: chrono::DateTime<Local>
}
