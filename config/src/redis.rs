pub use redis::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl RedisConfig{
    pub fn get_redis_url(&self)->String{
        self::generate_redis_conn_string(
            &self.username,
            &self.password,
            &self.port,
            &self.host,
            &self.database_name
        )

    }
}

fn generate_redis_conn_string(username: &String, password: &String, port: &u16, host: &String, database_name: &String) -> String {
    format!("redis://{username}:{password}@{host}:{port}/{database_name}")
}