use lanbase::{config, logger};
use serde::Deserialize;
use std::{error::Error, time::Duration};

#[derive(Deserialize)]
struct AppConfig {
    log: logger::LogConfig,
    storage: StorageConfig,
}

#[derive(Deserialize)]
struct StorageConfig {
    #[serde(with = "humantime_serde")]
    flush_interval: Duration,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args_os()
        .nth(1)
        .unwrap_or_else(|| "conf/app.toml".into());
    let settings: AppConfig = config::load(path)?;
    logger::init(&settings.log)?;
    let flush_interval_ms = u64::try_from(settings.storage.flush_interval.as_millis())?;
    tracing::info!(flush_interval_ms, "配置加载成功");
    Ok(())
}
