use std::fmt;

use ::tracing::info;
use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;
pub mod db;
pub mod redis;
pub mod http;
pub mod server;
pub mod profile;
pub mod mysql;


use self::{
    db::DatabaseConfig, http::HttpClientConfig, redis::RedisConfig,
    server::ServerConfig, profile::AppProfile , mysql::MysqlDatabaseConfig
};


pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::read().unwrap());

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db:DatabaseConfig,
    pub http: HttpClientConfig,
    pub redis: RedisConfig,
    pub profile:  AppProfile,
    pub mysqldb: MysqlDatabaseConfig
}



impl AppConfig {
    pub fn read() -> Result<Self, config::ConfigError> {
        let config_dir =
            utils::file::root_dir("appconfig").map_err(|e| ConfigError::Message(e.to_string()))?;
        let profile: AppProfile = std::env::var("APP_PROFILE")
            .unwrap_or_else(|_| "dev".into())
            .try_into()
            .map_err(ConfigError::Message)?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        info!("success read config profile: {profile}");
        config.try_deserialize()
    }
}




#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    pub use super::*;

    #[test]
    pub fn test_read_app_config() {
        let _config = AppConfig::read().unwrap();
    }

    #[test]
    pub fn test_profile_to_string() {
        let profile = AppProfile::try_from("Dev").unwrap();
        assert_eq!(profile, AppProfile::Dev)
    }
}