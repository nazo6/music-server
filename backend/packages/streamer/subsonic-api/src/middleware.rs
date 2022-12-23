use std::collections::HashMap;

use axum::{
    extract::Query,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub(crate) async fn auth<B>(
    Query(queries): Query<HashMap<String, String>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}

pub(crate) async fn auth_admin<B>(
    Query(queries): Query<HashMap<String, String>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}
