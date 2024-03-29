use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;
use user_service::{get_router, AppConfig};

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = AppConfig::default();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let app = get_router(config).await;

    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
