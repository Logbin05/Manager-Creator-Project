mod health;
mod users;

use axum::Router;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .nest("/users", users::router())
        .with_state(state)
}
