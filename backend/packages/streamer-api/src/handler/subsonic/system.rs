use axum::response::Response;
use serde::Serialize;

use super::Format;

#[derive(Serialize)]
pub(crate) struct PingResponse {}

pub(crate) async fn ping(format: Format) -> Response {
    format.render(PingResponse {})
}
