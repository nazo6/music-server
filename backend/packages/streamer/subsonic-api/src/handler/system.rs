use axum::response::Response;
use serde::Serialize;

use super::{Format, SubsonicErrorResponse};

#[derive(Serialize)]
pub(crate) struct PingResponse {}

pub(crate) async fn ping(format: Format) -> Result<Response, SubsonicErrorResponse> {
    Ok(format.render(PingResponse {}))
}
