use axum::routing::post;
use axum::Json;
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};
use server_core::access_token::new_access_token;

pub fn init() -> Router {
    Router::new().route("/login", post(login))
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
            let token = new_access_token(user.id).await;
            if let Ok(token) = token {
                Ok(Json(LoginResponseBody { token }))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
