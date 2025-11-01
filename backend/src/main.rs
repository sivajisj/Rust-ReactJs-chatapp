mod app;
mod config;
mod handlers;
mod routes;
mod services;
mod utils;

use crate::app::create_app;
use crate::config::AppConfig;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = AppConfig::init();
    let app = create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("🚀 Server running on http://{}", addr);

    // Modern Axum server setup
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}