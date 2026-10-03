use axum::{extract::State, http::StatusCode, routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(liveness))
        .route("/health/ready", get(readiness))
}

/// Процесс жив. Оркестратору этого хватает, чтобы не перезапускать под.
async fn liveness() -> &'static str {
    "ok"
}

/// Готов обслуживать: база отвечает.
async fn readiness(State(state): State<AppState>) -> (StatusCode, &'static str) {
    match sqlx::query("select 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, "ready"),
        Err(e) => {
            tracing::warn!(error = %e, "database is not reachable");
            (StatusCode::SERVICE_UNAVAILABLE, "database unavailable")
        }
    }
}
