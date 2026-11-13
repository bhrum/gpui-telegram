use gpui_telegram_server::{build_router, ServerConfig};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = ServerConfig::default();
    let app = build_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!("Starting gpui_telegram_server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
