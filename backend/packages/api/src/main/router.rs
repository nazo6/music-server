use axum::{body::Body, http::StatusCode, middleware::from_fn_with_state, routing::get, Router};
use common::AppState;

use super::middleware;

use super::handler::setup;

pub fn init(state: AppState) -> Router<AppState, Body> {
    let setup_routes = Router::new()
        .route("/set_admin", get(setup::set_admin))
        .route("/unimplemented", get(unimplemented_page))
        .layer(from_fn_with_state(state, middleware::setup_guard));

    Router::new().nest("/setup", setup_routes)
}

async fn unimplemented_page() -> Result<(), StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
