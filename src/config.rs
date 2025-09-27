use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    #[serde(default = "default_leptos_env")]
    pub leptos_env: String,
}

fn default_leptos_env() -> String {
    "DEV".to_string()
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?
            .try_deserialize::<Self>()?;
        Ok(cfg)
    }
}
