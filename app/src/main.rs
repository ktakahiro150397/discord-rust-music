use dotenvy::dotenv;
use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry_otlp::WithExportConfig;
use poise::serenity_prelude as serenity;
use std::env;
use std::time::Duration;
use tracing::{error, info, info_span};
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
    // Initialize Tracing
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::DEBUG)
    //     .with_timer(ChronoLocal::rfc_3339())
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_thread_ids(true)
    //     .with_thread_names(true)
    //     .init(); // 1. set Subscriber

    // event!(Level::INFO, "Event_1"); // 2. log Event_1
    // let _span1 = span!(Level::INFO, "Span_1").entered(); // 3. enter Span_1
    // event!(Level::INFO, "Event_2"); // 4. log Event_2
    // let span2 = span!(Level::INFO, "Span_2").entered(); // 5. enter Span_2
    // event!(Level::INFO, "Event_3"); // 6. log Event_3
    // span2.exit(); // 7. exit Span_2

    // event!(Level::TRACE, "trace");
    // event!(Level::DEBUG, "debug");
    // event!(Level::INFO, "info");
    // event!(Level::WARN, "warn");
    // event!(Level::ERROR, "error");

    // foo();

    // let test = Test {
    //     f1: "Hello".to_string(),
    //     f2: 42,
    //     f3: Some(42),
    // };
    // let test2 = Test {
    //     f1: "World".to_string(),
    //     f2: 42,
    //     f3: None,
    // };
    // event!(Level::DEBUG, test.f1, test.f2, test.f3); //フィールドを個別に出力

    // event!(Level::DEBUG, ?test); //フィールドを一括出力
    // event!(Level::DEBUG, ?test2); //フィールドを一括出力2
    // event!(Level::DEBUG, ?test, ?test2); //まとめて出力できる

    // foo2(9999);

    init_tracing();

    let version = env!("CARGO_PKG_VERSION");

    start()
        .instrument(info_span!("Straylight", version = version))
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    opentelemetry::global::shutdown_tracer_provider();

    // Launch
    // Straylight::run_test().await;
}

async fn start() {
    let user = "ymgyt";

    operation().instrument(info_span!("auth", %user)).await;
    operation_2().instrument(info_span!("db")).await;

    info!(
        ops = "xxx",
        counter.ops_count = 999,
        "successfully completed"
    );
}

async fn operation() {
    // trace
    // https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/struct.MetricsLayer.html#usage
    info!(
        ops = "xxx",
        counter.ops_count = 10,
        "successfully completed"
    );
}

async fn operation_2() {
    info!(arg = "xyz", "fetch resources...");
    error!("something went wrong");
}

fn build_metrics_controller() -> BasicController {
    opentelemetry_otlp::new_pipeline()
        .metrics(
            opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
            opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
            opentelemetry::runtime::Tokio,
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .build()
        .expect("Failed to build metrics controller")
}

fn init_tracing() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
                .with_resource(opentelemetry::sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", "sample-app"),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Not running in tokio runtime");

    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let otel_metrics_layer = tracing_opentelemetry::MetricsLayer::new(build_metrics_controller());

    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(otel_trace_layer)
        .with(otel_metrics_layer)
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}

#[tracing::instrument]
fn foo() {
    //let _span3 = span!(Level::INFO, "Span_3").entered(); // 8. enter Span_3
    event!(Level::INFO, "Event_4"); // 9. log Event_4
}

#[tracing::instrument]
fn foo2(arg: i32) {
    //let _span3 = span!(Level::INFO, "Span_3").entered(); // 8. enter Span_3
    println!("arg: {}", arg);
    event!(Level::INFO, "arg output!"); // 9. log Event_4
}

#[derive(Debug)]
struct Test {
    f1: String,
    f2: i32,
    f3: Option<i32>,
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
