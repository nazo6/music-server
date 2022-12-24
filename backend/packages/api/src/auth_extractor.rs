use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use server_core::access_token::validate_token;

#[derive(Debug)]
pub struct User(pub Option<entity::current::user::Model>);

pub struct ExtractUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await;
        if let Ok(TypedHeader(Authorization(token))) = token {
            let user = validate_token(token.token()).await;

            if let Ok(Some(user)) = user {
                Ok(ExtractUser(User(Some(user))))
            } else {
                Ok(ExtractUser(User(None)))
            }
        } else {
            Ok(ExtractUser(User(None)))
        }
    }
}
