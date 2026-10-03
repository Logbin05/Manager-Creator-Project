use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};

/// Поднимает пул и накатывает миграции из ./migrations.
pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    tracing::info!("migrations applied");

    Ok(pool)
}
