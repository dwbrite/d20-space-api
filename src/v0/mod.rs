use axum::Router;

mod maps;
mod meta;
mod users;

pub fn router() -> Router {
    Router::new()
}