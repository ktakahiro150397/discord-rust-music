use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::env;

mod commands;
mod playlist;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Launch
    Straylight::run_test().await;
}

struct Straylight {}

impl Straylight {
    async fn run_test() {
        let temp_path = std::path::PathBuf::from("temp");
        let mut playlist = playlist::playlist::PlayList::new();

        let track = match playlist::track::Track::from_youtube_url(
            &temp_path,
            "https://www.youtube.com/watch?v=pPoIneB_KLI&list=RDpPoIneB_KLI",
        )
        .await
        {
            Ok(t) => t,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        playlist.add(track);

        dbg!(playlist);
    }

    async fn run() {
        dotenv().expect(".env file not found!");

        let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment");
        let intents = serenity::GatewayIntents::non_privileged();

        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![
                    commands::test::age(),
                    commands::test::test(),
                    commands::test::download(),
                    commands::test::playlist(),
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
}
