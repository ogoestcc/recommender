mod database;
pub mod redis;
mod server;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub server: server::ServerConfig,
    pub db: database::DatabaseConfig,
    pub redis: redis::RedisConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
