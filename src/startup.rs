use axum::{routing::IntoMakeService, serve::Serve, Router};
use axum_login::{
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use deadpool_postgres::Pool;
use time::Duration;
use tokio::net::TcpListener;

use crate::{routes, users::Backend};

pub type App = Serve<IntoMakeService<Router>, Router>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub fn run(listener: TcpListener, pool: Pool) -> hyper::Result<App> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = Backend::new(pool.clone());
    let auth_service = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let state = AppState { pool };

    let app = Router::new()
        .merge(routes::router())
        .layer(auth_service)
        .with_state(state);

    Ok(axum::serve(listener, app.into_make_service()))
}
