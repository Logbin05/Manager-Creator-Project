use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};

use super::{domain::User, dto::CreateUser, repo};

/// Логика модуля. Другие модули работают с ним через этот тип,
/// а не через repo — так у модуля остаётся одна точка входа.
#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateUser) -> AppResult<User> {
        if !input.email.contains('@') {
            return Err(AppError::BadRequest("invalid email".into()));
        }
        if input.name.trim().is_empty() {
            return Err(AppError::BadRequest("name cannot be empty".into()));
        }

        let user = User::new(input.email, input.name);

        match repo::insert(&self.pool, &user).await {
            Ok(()) => Ok(user),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                Err(AppError::Conflict("email already taken".into()))
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get(&self, id: Uuid) -> AppResult<User> {
        repo::by_id(&self.pool, id).await?.ok_or(AppError::NotFound)
    }

    pub async fn list(&self, limit: i64) -> AppResult<Vec<User>> {
        Ok(repo::list(&self.pool, limit.clamp(1, 100)).await?)
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        if repo::delete(&self.pool, id).await? {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}
