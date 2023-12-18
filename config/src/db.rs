use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub max_connections: u32,
    pub database_name: String,
}

impl  DatabaseConfig{
    pub fn get_url(&self)->String{
        self::create_connection_string(
            &self.username,
            &self.password,
            &self.port,
            &self.host,
            &self.database_name

        )
    }
    pub fn check_if_db_parameters_is_empty(&self) ->bool{
        self::get_db_params(
            &self.username,
            &self.password,
            &self.port,
            &self.host,
            &self.database_name
        )
    }
    pub fn get_db_connection(&self)->PgConnectOptions {
        PgConnectOptions::new().host(&self.host).username(&self.username).password(&self.password)
            .port(self.port).database(&self.database_name).ssl_mode(PgSslMode::Prefer)

    }
}

fn get_db_params(username: &String, password: &String, port: &u16, host: &String, database_name: &String) -> bool {
    if username.trim().is_empty(){
        return  false
    }
    return  true
}

fn create_connection_string(username: &String, password: &String, port: &u16, host: &String, database_name: &String) -> String {
    format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
}