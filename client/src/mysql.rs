use async_trait::async_trait;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use sqlx::mysql::MySqlPool;

use config::mysql::MysqlDatabaseConfig;
pub type MysqlClient =MySqlPool;
#[async_trait]
pub trait MySqlPoolInit: Sized {
    fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error>;
    async fn get_version(&self) -> Result<Option<String>, sqlx::Error>;
}

#[async_trait]
impl MySqlPoolInit for MysqlClient {
    fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error> {
        sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.get_url())
    }

    async fn get_version(&self) -> Result<Option<String>, sqlx::Error> {
        let version: Option<String> = sqlx::query!(r#"SELECT version()"#)
            .fetch_one(self)
            .await
            .map(|r| r.version)?;
        Ok(version)
    }
}


async fn get_mysql_connection(mysql_options: &MySqlConnectOptions) -> sqlx::Result<sqlx::mysql::MySqlConnection> {
    MySqlConnectOptions::connect_with(mysql_options).await
}