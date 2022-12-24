use axum::Router;

mod auth;
mod auth_extractor;
mod graphql;

pub fn init() -> Router {
    let gql = Router::new().nest("/graphql", graphql::init());

    let auth = Router::new().nest("/auth", auth::init());

    Router::new().merge(gql).merge(auth)
}
