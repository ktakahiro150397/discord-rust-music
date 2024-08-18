use poise::serenity_prelude as serenity;

mod commands;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::test::age(),
                commands::test::test(),
                commands::test::download(),
                commands::utility::register(),
                commands::utility::help(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("::".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
