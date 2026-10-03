use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{CreateUser, User},
    services,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create).get(list))
        .route("/{id}", get(get_one).delete(delete))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 {
    20
}

// Хендлеры занимаются только HTTP: разбор входа, код ответа.
// Вся логика — в services.

async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<User>)> {
    let user = services::user::create(&state.db, input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<User>>> {
    let users = services::user::list(&state.db, params.limit).await?;
    Ok(Json(users))
}

async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<User>> {
    let user = services::user::get(&state.db, id).await?;
    Ok(Json(user))
}

async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<StatusCode> {
    services::user::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
