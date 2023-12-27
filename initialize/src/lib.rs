use client::{
    http::{HttpClient, HttpClientInit},
    postgres::{PgClient, PgPoolInit},
    mysql::{MysqlClient, MySqlPoolInit}

};
use config::AppConfig;
use errors::AppResult;

pub struct   InitializeApp{
    pub config:AppConfig,
    pub postgres:PgClient,
    pub http:HttpClient,
    pub mysql :MysqlClient
}


impl InitializeApp{
    pub async fn new(config :AppConfig)->AppResult<Self>{
        let postgres =PgClient::new(&config.db).await?;
        let mysql =MysqlClient::new(&config.mysqldb).await?;
        let http=HttpClient::build(&config.http).await?;

    Ok(Self{
        postgres,
        mysql,
        http,
        config
    })


    }
}