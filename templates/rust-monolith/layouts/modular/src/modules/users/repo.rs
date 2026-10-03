use sqlx::PgPool;
use uuid::Uuid;

use super::domain::User;

// query_as без макроса: проверка типов в рантайме, зато сборка не требует живой базы.
// Когда схема устоится — sqlx::query_as! + `cargo sqlx prepare`.

pub async fn insert(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into users (id, email, name, created_at, updated_at)
         values ($1, $2, $3, $4, $5)",
    )
    .bind(user.id)
    .bind(&user.email)
    .bind(&user.name)
    .bind(user.created_at)
    .bind(user.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("select * from users where id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool, limit: i64) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("select * from users order by created_at desc limit $1")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("delete from users where id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
