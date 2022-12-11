use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::Response,
};
use serde::{Deserialize, Serialize};

pub(crate) mod system;

pub(crate) struct SubsonicErrorResponse {
    code: usize,
    message: String,
}

pub(crate) enum Format {
    Json,
    Xml,
}
impl Format {
    fn render<T>(self, data: T) -> Response
    where
        T: Serialize,
    {
        match self {
            Format::Json => {
                todo!("render json...")
            }
            Format::Xml => {
                todo!("render xml...")
            }
        }
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for Format
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct FormatQuery {
            f: String,
        }

        let Query(query) = match Query::<FormatQuery>::from_request_parts(parts, state).await {
            Ok(query) => query,
            Err(_) => return Ok(Self::Xml),
        };

        if query.f == "json" {
            Ok(Self::Json)
        } else {
            Ok(Self::Xml)
        }
    }
}
