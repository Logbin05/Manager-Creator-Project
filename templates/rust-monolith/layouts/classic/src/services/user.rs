use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::{CreateUser, User},
    repositories,
};

pub async fn create(pool: &PgPool, input: CreateUser) -> AppResult<User> {
    if !input.email.contains('@') {
        return Err(AppError::BadRequest("invalid email".into()));
    }
    if input.name.trim().is_empty() {
        return Err(AppError::BadRequest("name cannot be empty".into()));
    }

    let now = Utc::now();
    let user = User {
        // v7 упорядочен по времени: вставки идут в конец индекса, а не размазываются по нему
        id: Uuid::now_v7(),
        email: input.email,
        name: input.name,
        created_at: now,
        updated_at: now,
    };

    match repositories::user::insert(pool, &user).await {
        Ok(()) => Ok(user),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            Err(AppError::Conflict("email already taken".into()))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn get(pool: &PgPool, id: Uuid) -> AppResult<User> {
    repositories::user::by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn list(pool: &PgPool, limit: i64) -> AppResult<Vec<User>> {
    Ok(repositories::user::list(pool, limit.clamp(1, 100)).await?)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
    if repositories::user::delete(pool, id).await? {
        Ok(())
    } else {
        Err(AppError::NotFound)
    }
}
