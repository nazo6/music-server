use axum::routing::post;
use axum::Json;
use axum::{http::StatusCode, Router};
use serde::{Deserialize, Serialize};
use server_core::access_token::{create_token, revoke_token};

use crate::AppState;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
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
            let token = create_token(user.id).await;
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

#[derive(Debug, Deserialize)]
struct LogoutRequestBody {
    token: String,
}
#[derive(Debug, Serialize)]
struct LogoutResponseBody {
    message: String,
}
async fn logout(
    Json(body): Json<LogoutRequestBody>,
) -> Result<Json<LogoutResponseBody>, StatusCode> {
    let res = revoke_token(&body.token).await;

    match res {
        Ok(_) => Ok(Json(LogoutResponseBody {
            message: "Successfully logged out".to_string(),
        })),
        Err(server_core::errors::Error::GeneralError(_message)) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
