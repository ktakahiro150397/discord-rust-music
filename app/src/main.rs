use dotenvy::dotenv;
use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry_otlp::WithExportConfig;
use poise::serenity_prelude as serenity;
use std::env;
use std::time::Duration;
use tracing::{debug, error, info, info_span, trace};
use tracing::{event, span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::fmt::time::ChronoLocal;

mod commands;
mod playlist;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found!");

    // Initialize Tracing
    let otel_endpoint = std::env::var("OTEL_ENDPOINT").unwrap_or("".to_string());
    println!("OTEL_ENDPOINT: {}", otel_endpoint);
    init_tracing(&otel_endpoint);

    // Launch
    Straylight::run().await;

    // Shutdown
    tokio::time::sleep(Duration::from_secs(30)).await;
    opentelemetry::global::shutdown_tracer_provider();
}

fn build_metrics_controller(otel_endpoint: &str) -> BasicController {
    opentelemetry_otlp::new_pipeline()
        .metrics(
            opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
            opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
            opentelemetry::runtime::Tokio,
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otel_endpoint),
        )
        .build()
        .expect("Failed to build metrics controller")
}

fn init_tracing(otel_endpoint: &str) {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otel_endpoint),
        )
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
                .with_resource(opentelemetry::sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", "rust-music-bot"),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Not running in tokio runtime");

    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let otel_metrics_layer =
        tracing_opentelemetry::MetricsLayer::new(build_metrics_controller(&otel_endpoint));

    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(otel_trace_layer)
        .with(otel_metrics_layer)
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}

struct Straylight {}

impl Straylight {
    async fn run_test() {
        let span = span!(Level::INFO, "run_test_span");
        let _enter = span.enter();

        let version = env!("CARGO_PKG_VERSION");
        info!("Starting Rust Music Bot v{} / run_test", version);

        let rust_log = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        info!("RUST_LOG: {}", rust_log);

        // trace!("trace message");
        // info!("info message");
        // debug!("debug message");
        // error!("error message");

        let temp_path = std::path::PathBuf::from("temp");
        let mut playlist = playlist::playlist::PlayList::new();

        let track = match playlist::track::Track::from_youtube_url(
            &temp_path,
            "https://www.youtube.com/watch?v=pPoIneB_KLI&list=RDpPoIneB_KLI",
        )
        .instrument(info_span!("get track"))
        .await
        {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        playlist.add(track);

        let track = match playlist::track::Track::from_youtube_url(
            &temp_path,
            "https://www.youtube.com/watch?v=abcdef",
        )
        .instrument(info_span!("get track"))
        .await
        {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        playlist.add(track);
    }

    async fn run() {
        let span = span!(Level::INFO, "run_app");
        let _enter = span.enter();

        let version = env!("CARGO_PKG_VERSION");
        info!("Starting Rust Music Bot v{}", version);

        let rust_log = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        info!("RUST_LOG: {}", rust_log);

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

        client
            .unwrap()
            .start()
            .instrument(info_span!("serenity_client"))
            .await
            .unwrap();
    }
}
