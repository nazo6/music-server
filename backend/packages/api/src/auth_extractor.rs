use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use server_core::access_token::validate_token;

pub type User = entity::current::user::Model;

pub struct ExtractUser(pub Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await;
        if let Ok(TypedHeader(Authorization(token))) = token {
            let user = validate_token(token.token()).await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error validating token. Please try again.",
                )
            })?;

            if let Some(user) = user {
                Ok(ExtractUser(Some(user)))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid token"))
            }
        } else {
            // No token provided (guest)
            Ok(ExtractUser(None))
        }
    }
}
