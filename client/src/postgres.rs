use async_trait::async_trait;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tracing::info;

use config::db::DatabaseConfig;
use errors::AppResult;

pub type PgClient = PgPool;

#[async_trait]
pub trait PgPoolExt: Sized {
    async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error>;
}

#[async_trait]
impl PgPoolExt for PgPool {
    async fn new(config: &DatabaseConfig) -> Result<PgClient, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.get_url())
            .await
    }

}

async fn get_pg_connection(pg_options: &PgConnectOptions) -> sqlx::Result<PgConnection> {
    PgConnection::connect_with(pg_options).await
}



