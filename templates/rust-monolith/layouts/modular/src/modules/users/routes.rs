use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{error::AppResult, shared::AppState};

use super::{
    domain::User,
    dto::{CreateUser, ListParams},
    service::UserService,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create).get(list))
        .route("/{id}", get(get_one).delete(delete))
}

async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<User>)> {
    let user = UserService::new(state.db).create(input).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<User>>> {
    let users = UserService::new(state.db).list(params.limit).await?;
    Ok(Json(users))
}

async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<User>> {
    let user = UserService::new(state.db).get(id).await?;
    Ok(Json(user))
}

async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<StatusCode> {
    UserService::new(state.db).delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
