mod config;
use config::AppConfig;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = AppConfig::from_file("config/local")?;
    info!("Config loaded: {:?}", cfg);

    Ok(())
}
