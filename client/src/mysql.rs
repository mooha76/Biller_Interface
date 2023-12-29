use async_trait::async_trait;


use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{Connection, Error, Executor, MySqlConnection, MySqlPool};

use tracing::info;

use config::db::DatabaseConfig;
use config::mysql::MysqlDatabaseConfig;
use errors::AppResult;

pub type MySqlPoolClient = MySqlPool;



#[async_trait]
pub trait MySqlPoolExt: Sized {
    async fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error>;
}

#[async_trait]
impl MySqlPoolExt for MySqlPool {
    async fn new(config: &MysqlDatabaseConfig) -> Result<Self, sqlx::Error> {
        MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.get_url())
            .await
    }
}

async fn get_mysql_connection(mysql_options: &MySqlConnectOptions) -> sqlx::Result<MySqlConnection> {
    MySqlConnection::connect_with(mysql_options).await
}