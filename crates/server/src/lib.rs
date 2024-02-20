mod routes;

use std::net::SocketAddr;

use anyhow::{Context, Result};
use tokio::net::TcpListener;

pub struct Config {
    pub address: SocketAddr,
}

pub async fn serve(config: Config) -> Result<()> {
    tracing::info!("Starting server at {}", config.address);

    let tcp = TcpListener::bind(config.address)
        .await
        .context("Failed to bind TCP listener")?;

    axum::serve(tcp, routes::router())
        .await
        .context("Failed to serve")?;

    Ok(())
}
