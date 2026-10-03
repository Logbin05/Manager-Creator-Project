use sqlx::PgPool;

/// Всё, что хендлеры получают через `State`.
/// Клонируется на каждый запрос, поэтому внутри только дешёвые в клонировании вещи
/// (PgPool — это Arc внутри).
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
