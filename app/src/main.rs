use opentelemetry::sdk::metrics::controllers::BasicController;
use std::env;
use std::time::Duration;
use tracing::{error, info, info_span};
use tracing::{span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::util::SubscriberInitExt;

mod playlist;
mod rust_lang;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found!");

    // Initialize Tracing
    let otel_endpoint = std::env::var("OTEL_ENDPOINT").unwrap_or("".to_string());
    println!("OTEL_ENDPOINT: {}", otel_endpoint);
    init_tracing(&otel_endpoint);

    // Launch
    Straylight::run_test().await;

    // Shutdown
    tokio::time::sleep(Duration::from_secs(3)).await;
    opentelemetry::global::shutdown_tracer_provider();
}

// fn build_metrics_controller(otel_endpoint: &str) -> BasicController {
//     opentelemetry_otlp::new_pipeline()
//         .metrics(
//             opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
//             opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
//             opentelemetry::runtime::Tokio,
//         )
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .tonic()
//                 .with_endpoint(otel_endpoint),
//         )
//         .build()
//         .expect("Failed to build metrics controller")
// }

fn init_tracing(_otel_endpoint: &str) {
    // let tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     // .with_exporter(
    //     //     opentelemetry_otlp::new_exporter()
    //     //         .tonic()
    //     //         .with_endpoint(otel_endpoint),
    //     // )
    //     .with_trace_config(
    //         opentelemetry::sdk::trace::config()
    //             .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
    //             .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
    //             .with_resource(opentelemetry::sdk::Resource::new(vec![
    //                 opentelemetry::KeyValue::new("service.name", "rust-music-bot"),
    //             ])),
    //     )
    //     .install_batch(opentelemetry::runtime::Tokio)
    //     .expect("Not running in tokio runtime");

    // let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // let otel_metrics_layer =
    //     tracing_opentelemetry::MetricsLayer::new(build_metrics_controller(&otel_endpoint));

    // use tracing_subscriber::layer::SubscriberExt;
    // use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::Registry::default().init();
    //.with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
    // .with(otel_trace_layer)
    // .with(otel_metrics_layer)
    //.with(tracing_subscriber::filter::LevelFilter::INFO)
    //.init();
}

struct Straylight {}

impl Straylight {
    async fn run_test() {
        let version = env!("CARGO_PKG_VERSION");
        println!("Starting Rust Music Bot v{} / run_test", version);

        let rust_log = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        println!("RUST_LOG: {}", rust_log);

        let test_list = vec![1, 2, 3, 4, 5];
        let largest = rust_lang::no_generics::largest_i32(&test_list);
        println!("Largest i32: {}", largest);

        let largest = rust_lang::generics::largest(&test_list);
        println!("Largest i32 generics: {}", largest);

        let test_list = vec!['1', 'a'];
        let largest_char = rust_lang::no_generics::largest_char(&test_list);
        println!("Largest char: {}", largest_char);
        println!("Data: {:?}", test_list);

        let largest = rust_lang::generics::largest(&test_list);
        println!("Largest i32 generics: {}", largest);
        println!("Data: {:?}", test_list);

        let largest = rust_lang::generics::largest_ref(&test_list);
        println!("Largest i32 generics ref: {}", largest);
        println!("Data: {:?}", test_list);

        println!("Program end");
    }
}
