use axum::Router;

mod auth;
mod auth_extractor;
mod graphql;
mod setup;

pub fn init() -> Router {
    let gql = Router::new().nest("/graphql", graphql::init());

    let rest = Router::new()
        .nest("/auth", auth::init())
        .nest("/setup", setup::init());

    Router::new().merge(gql).merge(rest)
}
