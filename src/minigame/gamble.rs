pub mod handle_gamble;
pub mod handle_register;

const DEFAULT_TOKENS: i64 = 10000;
const DEFAULT_RATE: i16 = 5000; // 50%
const DEFAULT_CRIT_RATE: i16 = 0; // 50%
const DEFAULT_CRIT_MUL: i32 = 10000; // 100%
const DEFAULT_REVIVE_TOKENS: i32 = 10000; // 100%
const DEFAULT_AUTO_REVIVE: bool = false;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
struct User {
    id: i64,
    tokens: i64,
    rate: i16,
    crit_rate: i16,
    crit_mul: i32,
    revive_tokens: i64,
    auto_revive: bool,
}
