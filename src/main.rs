// rcli csv -i input.csv -o output.json --header -d ','

use chrono::Local;
use clap::Parser;
use tracing_subscriber::fmt::time::FormatTime;
use rust_web::{CmdExector, Opts, Settings, load_config}; //
use tracing_subscriber::fmt::format::Writer;
use std::fmt::Result as FmtResult;
use tracing_subscriber::{fmt, filter::{EnvFilter, LevelFilter}, Registry};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing::{debug, info, Level};
use std::env;
use tracing_subscriber::layer::SubscriberExt;

mod config;

#[derive(Clone, Copy)]
struct CustomTime;

impl FormatTime for CustomTime {
    fn format_time(&self, w: &mut Writer<'_>) -> FmtResult {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Parse command-line arguments
    let opts = Opts::parse();

    // Load configuration file
    let settings = load_config(&opts.env)?;

    // Create a rolling file appender
    let file_appender = RollingFileAppender::new(Rotation::NEVER, "/Users/xiaguang/httx", "out.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Create a custom time formatter
    let timer = CustomTime;

    // Set log level based on configuration
    let log_level = match settings.log_level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        _ => Level::INFO, // Default level
    };

    // Create a logging layer for outputting to file
    let file_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(non_blocking)
        .with_ansi(false); // Disable ANSI colors for file output

    // Create a logging layer for outputting to console
    let console_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(std::io::stdout)
        .with_ansi(true); // Enable ANSI colors for console output

    let base_subscriber = Registry::default()
        .with(file_layer)
        .with(EnvFilter::new(format!("{}={}", module_path!(), log_level)));

    let subscriber: Box<dyn tracing::Subscriber + Send + Sync> = if opts.env == "dev" {
        Box::new(base_subscriber.with(console_layer))
    } else {
        Box::new(base_subscriber)
    };


    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global subscriber");


    debug!("Running in {} mode", opts.env);
    info!("This is an info message with custom time format");
    tracing::debug!("Loaded configuration: {:?}", settings);

    opts.cmd.execute(&settings).await?;

    Ok(())
}
