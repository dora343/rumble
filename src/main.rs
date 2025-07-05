use poise::serenity_prelude as serenity;
use sqlx::postgres::PgPoolOptions;
mod cmd;
mod event_handler;
mod minigame;

struct Data {
    dbpool: sqlx::PgPool,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let cmd_prefix = std::env::var("COMMAND_PREFIX").expect("missing COMMAND_PREFIX");
    let _bot_owner_id = std::env::var("BOT_OWNER_ID").expect("missing BOT_OWNER_ID");

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(cmd_prefix.clone()),
                case_insensitive_commands: false,
                ..Default::default()
            },
            commands: vec![
                cmd::age::age(),
                cmd::recursion_test::recursion_test(),
                cmd::help::help(),
                cmd::register::register(),
                cmd::gamble::gamble(),
                cmd::revive::revive(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    586966462527307796.into(),
                )
                .await?;
                Ok(Data { dbpool: pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
