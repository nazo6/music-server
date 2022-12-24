use axum::routing::post;
use axum::Json;
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};

pub fn init() -> Router {
    Router::new().route("/", post(login))
}

#[derive(Debug, Deserialize)]
struct LoginRequestBody {
    username: String,
    password: String,
}
#[derive(Debug, Serialize)]
struct LoginResponseBody {
    token: String,
}
async fn login(Json(body): Json<LoginRequestBody>) -> Result<Json<LoginResponseBody>, StatusCode> {
    let user = server_core::user::get_user_if_authed(&body.username, &body.password).await;

    match user {
        Ok(Some(user)) => {
            let token = uuid::Uuid::new_v4().to_string();
            Ok(Json(LoginResponseBody { token }))
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
