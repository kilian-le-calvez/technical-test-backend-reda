use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpstreamConfig {
    pub python_api_base_url: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub upstream: UpstreamConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::new(path, config::FileFormat::Toml));
        let cfg = builder.build()?;
        cfg.try_deserialize()
    }
}
