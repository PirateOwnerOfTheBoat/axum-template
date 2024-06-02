use axum::Router;

use crate::startup::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}
