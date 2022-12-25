use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Schema,
};
use async_graphql_axum::{
    GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLSubscription, GraphQLWebSocket,
};
use axum::{
    extract::{Extension, State, WebSocketUpgrade},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use server_core::access_token::validate_token;
use tracing::log::debug;

use crate::{auth_extractor::ExtractUser, graphql::schema::guard::Role, AppState};

use self::schema::ApiSchema;

mod schema;

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("ws://localhost:3000/graphql/ws")
            .finish(),
    )
}

async fn graphql_handler(
    Extension(schema): Extension<ApiSchema>,
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
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
    req = req.data(state);

    schema.execute(req).await.into()
}

async fn graphql_subscription_handler(
    Extension(schema): Extension<ApiSchema>,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
    protocol: GraphQLProtocol,
) -> Response {
    let mut data = async_graphql::Data::default();
    data.insert(state);

    let resp = ws
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .on_connection_init(|value| async move {
                    #[derive(Deserialize)]
                    struct Payload {
                        token: String,
                    }
                    let value = serde_json::from_value::<Payload>(value)
                        .map_err(|_| "Please provide token".to_string())?;

                    let user = validate_token(&value.token)
                        .await
                        .map_err(|_| "Invalid token".to_string())?;

                    let role = if let Some(user) = &user {
                        if user.is_admin {
                            Role::Admin
                        } else {
                            Role::User
                        }
                    } else {
                        Role::Guest
                    };

                    data.insert(user);
                    data.insert(role);

                    Ok(data)
                })
                .serve()
        });
    resp.into_response()
}

pub fn init() -> Router<AppState> {
    let schema = Schema::new(
        schema::query::QueryRoot,
        schema::mutation::MutationRoot,
        schema::subscription::Subscription,
    );
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", get(graphql_subscription_handler))
        .layer(Extension(schema))
}
