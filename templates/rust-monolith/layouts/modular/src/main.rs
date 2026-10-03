mod bootstrap;
mod modules;
mod shared;

// Общая инфраструктура: её видят все модули, она не знает ни об одном из них.
mod config;
mod db;
mod error;
mod telemetry;

use anyhow::Context;
use tokio::net::TcpListener;

use crate::{config::Config, shared::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    telemetry::init();

    let config = Config::from_env()?;
    let db = db::connect(&config.database_url).await?;

    let app = bootstrap::app(AppState { db });

    let listener = TcpListener::bind(config.addr)
        .await
        .with_context(|| format!("cannot bind {}", config.addr))?;
    tracing::info!(addr = %config.addr, "listening");

    axum::serve(listener, app).await?;
    Ok(())
}
