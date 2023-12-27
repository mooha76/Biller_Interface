use async_trait::async_trait;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};

use sqlx::mysql::MySqlPool;

use config::mysql::MysqlDatabaseConfig;
pub type MysqlClient =MySqlPoolOptions;
#[async_trait]

pub trait MySqlPoolInit: Sized {
    fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error>;
}

#[async_trait]
impl MySqlPoolInit for MysqlClient {
    fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(config.get_options());
        Ok(pool)
    }
}

async fn get_mysql_connection(mysql_options: &MySqlConnectOptions) -> sqlx::Result<sqlx::mysql::MySqlConnection> {
    sqlx::mysql::MySqlConnection::connect_with(mysql_options).await
}