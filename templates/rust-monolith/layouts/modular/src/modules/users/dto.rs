use serde::Deserialize;

/// Форма запроса живёт внутри модуля: наружу она не нужна.
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    20
}
