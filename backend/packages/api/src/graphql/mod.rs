use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, Extension},
    http::header::HeaderMap,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router, Server,
};

use self::schema::ApiSchema;

mod schema;

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    Extension(schema): Extension<ApiSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub fn init() -> Router {
    let schema = Schema::new(schema::QueryRoot, schema::MutationRoot, EmptySubscription);
    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}
