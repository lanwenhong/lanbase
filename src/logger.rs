use serde::Deserialize;
use tracing_subscriber::{filter::LevelFilter, util::TryInitError};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
    pub level: LogLevel,
    pub format: LogFormat,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

/// Installs the global tracing subscriber. Call once, from the application.
pub fn init(config: &LogConfig) -> Result<(), TryInitError> {
    let level = match config.level {
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Warn => LevelFilter::WARN,
        LogLevel::Error => LevelFilter::ERROR,
    };
    let subscriber = tracing_subscriber::fmt().with_max_level(level);
    use tracing_subscriber::util::SubscriberInitExt;
    match config.format {
        LogFormat::Text => subscriber.finish().try_init(),
        LogFormat::Json => subscriber.json().finish().try_init(),
    }
}
