use axum::routing::post;
use axum::Json;
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};

use crate::AppState;

pub fn init() -> Router<AppState> {
    Router::new().route("/create_admin", post(create_admin))
}

#[derive(Debug, Deserialize)]
struct CreateAdminRequestBody {
    username: String,
    password: String,
}
#[derive(Debug, Serialize)]
struct CreateAdminResponseBody {
    token: String,
}
async fn create_admin(
    Json(body): Json<CreateAdminRequestBody>,
) -> Result<Json<CreateAdminResponseBody>, StatusCode> {
    let token = server_core::setup::create_admin_user(&body.username, &body.password).await;

    match token {
        Ok(Some(token)) => Ok(Json(CreateAdminResponseBody { token })),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
