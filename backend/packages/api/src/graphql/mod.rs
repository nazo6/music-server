use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::log::debug;

use crate::{auth_extractor::ExtractUser, graphql::schema::guard::Role};

use self::schema::ApiSchema;

mod schema;

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    Extension(schema): Extension<ApiSchema>,
    ExtractUser(user): ExtractUser,
    req: GraphQLRequest,
) -> GraphQLResponse {
    debug!("graphql_handler: {:?}", user);

    let role = if let Some(user) = &user {
        if user.is_admin {
            Role::Admin
        } else {
            Role::User
        }
    } else {
        Role::Guest
    };

    let mut req = req.into_inner();
    req = req.data(user);
    req = req.data(role);

    schema.execute(req).await.into()
}

pub fn init() -> Router {
    let schema = Schema::new(
        schema::query::QueryRoot,
        schema::mutation::MutationRoot,
        EmptySubscription,
    );
    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}
