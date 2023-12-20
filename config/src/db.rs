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
        self::is_db_params_is_empty(
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

fn is_db_params_is_empty(username: &String, password: &String, port: &u16, host: &String, database_name: &String) -> bool {
    if username.trim().is_empty(){
        return  false
    }
    return  true
}

fn create_connection_string(username: &String, password: &String, port: &u16, host: &String, database_name: &String) -> String {
    format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
}


#[cfg(test)]
 pub  mod test{
    use super::*;
    #[test]
    pub fn db_string_connection_test(){
        let config=DatabaseConfig{
            username: "Mooha".to_string(),
            password: "cade".to_string(),
            port: 3663,
            host: "127.0.0.1".to_string(),
            max_connections: 0,
            database_name: "myDB".to_string(),
        };
        assert_eq!(config.get_url(),"postgres://Mooha:cade@127.0.0.1:3663/myDB")
    }
    #[test]
    pub fn is_db_string_connection_empty_test(){
        let config =DatabaseConfig{
            username: "".to_string(),
            password: "".to_string(),
            port: 0,
            host: "".to_string(),
            max_connections: 0,
            database_name: "".to_string(),
        };
        assert_eq!(config.check_if_db_parameters_is_empty(),false)

    }
}
