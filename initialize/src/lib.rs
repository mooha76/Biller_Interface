use client::{
    http::{HttpClient, HttpClientInit},
    postgres::{PgClient , PgPoolExt},
    mysql::{MySqlPoolClient , MySqlPoolExt}

};
use config::AppConfig;
use errors::AppResult;

pub struct   InitializeApp{
    pub config:AppConfig,
    pub postgres:PgClient,
    pub http:HttpClient,
    pub mysql :MySqlPoolClient
}


impl InitializeApp{
    pub async fn new(config :AppConfig)->AppResult<Self>{
        let postgres =PgClient::new(&config.db).await?;
        let mysql =MySqlPoolClient::new(&config.mysqldb).await?;
        let http = HttpClient::build(&config.http)?;

    Ok(Self{
        postgres,
        mysql,
        http,
        config
    })


    }
}