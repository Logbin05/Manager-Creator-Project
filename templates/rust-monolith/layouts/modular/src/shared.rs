use sqlx::PgPool;

/// Общее состояние. Модули берут отсюда то, что им нужно,
/// и не держат ссылок друг на друга.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
