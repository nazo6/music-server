use axum::Router;

pub mod auth;
pub mod graphql;

pub fn init() -> Router {
    let app = Router::new().nest("/graphql", graphql::init());

    let auth = Router::new().nest("/login", auth::init());
    app
}
