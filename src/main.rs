mod config;
mod db;
mod models;
mod routes;

use axum::Router;
use config::Config;
use db::create_pool;
use routes::api_tests;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env();
    let pool = create_pool(&cfg).await;

    let app = Router::new().merge(api_tests::routes()).with_state(pool);

    let addr = format!("0.0.0.0:{}", cfg.server_port);
    tracing::info!("🚀 Server running at {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
