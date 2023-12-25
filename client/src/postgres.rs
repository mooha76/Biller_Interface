use async_trait::async_trait;
use sqlx::postgres::{
    PgConnectOptions, PgPoolOptions
};
use sqlx::{Connection, Error, Executor, PgConnection, PgPool};

use tracing::info;

use config::db::DatabaseConfig;
use errors::AppResult;

pub type PgClient=PgPool;

#[async_trait]

pub trait PgPoolInit :Sized{
    fn new(config:&DatabaseConfig)->Result<Self , sqlx::Error>;
    async fn getversion(&self) -> Result<Option<String>, sqlx::Error>;
}

#[async_trait]

impl PgPoolInit for PgClient{
    async fn new(config: &DatabaseConfig) -> Result<Self, Error> {
        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.get_url())
            .await
    }
    async fn getversion(&self) -> Result<Option<String>, sqlx::Error> {
        let version: Option<String> = sqlx::query!(r#"SELECT version()"#)
            .fetch_one(self)
            .await
            .map(|r| r.version)?;
        Ok(version)
    }

}


async fn get_pg_connection(pg_options: &PgConnectOptions) -> sqlx::Result<PgConnection> {
    PgConnection::connect_with(pg_options).await
}



#[cfg(test)]
mod tests {
    use super::*;
    use config::CONFIG;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_postgres_connection() {
        let client = PgPool::new(&CONFIG.db).await.unwrap();
        let version = client.getversion().await.unwrap().unwrap();
        assert!(!version.is_empty())
    }
}