use poise::serenity_prelude as serenity;

mod cmd;
mod event_handler;
mod minigame;

struct Data {
    cmd_prefix: String,
    bot_owner_id: String,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let cmd_prefix = std::env::var("COMMAND_PREFIX").expect("missing COMMAND_PREFIX");
    let bot_owner_id = std::env::var("BOT_OWNER_ID").expect("missing BOT_OWNER_ID");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

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
                cmd::gamble::gamble(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { 
                    cmd_prefix, 
                    bot_owner_id, 
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
