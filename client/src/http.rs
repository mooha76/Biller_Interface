use std::time::Duration;

use async_trait::async_trait;

use config::http::HttpClientConfig;

pub type HttpClient = reqwest::Client;

#[async_trait]
pub trait HttpClientInit {
    fn build(config: &HttpClientConfig) -> Result<reqwest::Client, reqwest::Error>;
}

#[async_trait]
impl HttpClientInit for HttpClient {
    fn build(config: &HttpClientConfig) -> Result<Self, reqwest::Error> {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
    }
}

