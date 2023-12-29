use axum::{
    routing::get,
    Router,
    Server as AxumServer,
};
use config::AppConfig;
use errors::AppResult;
use initialize::InitializeApp;
use std::net::TcpListener;
use tracing::info;

pub struct Server {
    pub initialize: InitializeApp,
    pub listener: TcpListener,
}

impl Server {
    pub async fn new(mut config: AppConfig) -> AppResult<Self> {
        let listener = TcpListener::bind(config.server.get_server_addrs())?;
        config.server.port = listener.local_addr()?.port();
        let state = InitializeApp::new(config).await?;
        Ok(Self { initialize: state, listener })
    }
    pub async fn run(self) -> std::io::Result<()> {
        info!(
            "run server: {} profile: {}",
            self.initialize.config.server.get_server_addrs(),
            self.initialize.config.profile
        );

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }));

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}