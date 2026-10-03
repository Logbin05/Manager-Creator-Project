mod domain;
mod dto;
mod repo;
mod routes;
mod service;

use axum::Router;

use crate::shared::AppState;

// ── Публичный API модуля ──
// Наружу видно только то, что перечислено здесь. Всё остальное приватно,
// и компилятор не даст другому модулю залезть в repo или dto напрямую.

// Пока модуль один, этими именами никто не пользуется — отсюда allow.
// Как только появится второй модуль, он будет ходить сюда и только сюда.
#[allow(unused_imports)]
pub use self::{domain::User, service::UserService};

pub fn router() -> Router<AppState> {
    routes::router()
}
