use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            // v7 упорядочен по времени: вставки идут в конец индекса
            id: Uuid::now_v7(),
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}
