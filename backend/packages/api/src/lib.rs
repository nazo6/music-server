use axum::Router;
use common::AppState;

pub mod app;
pub mod auth;
pub mod graphql;

pub fn init(state: AppState) -> Router {
    let app = Router::new()
        .nest("/graphql", graphql::init())
        .with_state(state);

    let auth = Router::new().nest("/login", auth::init());
    app
}
