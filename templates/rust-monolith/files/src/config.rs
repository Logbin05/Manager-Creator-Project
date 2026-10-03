use std::{env, net::SocketAddr};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub addr: SocketAddr,
}

impl Config {
    /// Читает настройки из окружения. Значения из .env подхватывает dotenvy в main.
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;

        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".into());
        let addr = format!("{host}:{port}")
            .parse()
            .with_context(|| format!("invalid address: {host}:{port}"))?;

        Ok(Self { database_url, addr })
    }
}
