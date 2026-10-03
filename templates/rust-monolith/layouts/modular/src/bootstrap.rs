use axum::{extract::State, http::StatusCode, routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{modules, shared::AppState};

/// Единственное место, где модули собираются в приложение.
/// Новый модуль — одна строка здесь.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(liveness))
        .route("/health/ready", get(readiness))
        .nest("/users", modules::users::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn liveness() -> &'static str {
    "ok"
}

async fn readiness(State(state): State<AppState>) -> (StatusCode, &'static str) {
    match sqlx::query("select 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, "ready"),
        Err(e) => {
            tracing::warn!(error = %e, "database is not reachable");
            (StatusCode::SERVICE_UNAVAILABLE, "database unavailable")
        }
    }
}
