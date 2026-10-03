mod config;
mod db;
mod error;
mod models;
mod repositories;
mod routes;
mod services;
mod state;
mod telemetry;

use anyhow::Context;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    telemetry::init();

    let config = Config::from_env()?;
    let db = db::connect(&config.database_url).await?;

    let app = routes::router(AppState { db }).layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(config.addr)
        .await
        .with_context(|| format!("cannot bind {}", config.addr))?;
    tracing::info!(addr = %config.addr, "listening");

    axum::serve(listener, app).await?;
    Ok(())
}
