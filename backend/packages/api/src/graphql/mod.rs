use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::Extension,
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::log::debug;

use crate::auth_extractor::ExtractUser;

use self::schema::ApiSchema;

mod schema;

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    Extension(schema): Extension<ApiSchema>,
    ExtractUser(user): ExtractUser,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    debug!("graphql_handler: {:?}", user);
    let mut req = req.into_inner();
    req = req.data(user);
    schema.execute(req).await.into()
}

pub fn init() -> Router {
    let schema = Schema::new(schema::QueryRoot, schema::MutationRoot, EmptySubscription);
    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}
