
use config::{self,logtracer, AppConfig};
use errors::AppResult;
use server::Server;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (file_appender, _file_appender_guard) = tracing_appender::non_blocking(file_appender);
    config::logtracer::init_subscriber(config::logtracer::create_subscriber(
        "app",
        EnvFilter::from_default_env(),
        file_appender,
    ))?;
    info!("init tracing success");
    let config = AppConfig::read()?;
    info!("read config file success");
    let server = Server::new(config.clone()).await?;
    info!("create server");
    let state = server.initialize.clone();
    let server = server.run().await?;
    info!("run server");
    let server_task = tokio::task::spawn(server);
    info!("spawn server task");
    let _ = server_task.await;
    Ok(())
}
