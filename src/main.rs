mod config;
mod core;
mod db;
mod handler;
mod models;
mod routes;
mod services;

use axum::Router;
use config::Config;
use db::create_pool;
use routes::api_tests;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::calendar;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env();
    let pool = create_pool(&cfg).await;

    let app = Router::new()
        .merge(api_tests::routes())
        .merge(routes::server_list::routes())
        .merge(calendar::routes())
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", 3000);
    tracing::info!("🚀 Server running at {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
