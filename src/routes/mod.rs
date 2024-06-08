use axum::Router;

use crate::startup::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest_service("/public", ServeDir::new("public"))
}
