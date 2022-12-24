use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use common::AppState;
use entity::current::*;
use sea_orm::{EntityTrait, PaginatorTrait};
use tracing::log::*;

pub async fn setup_guard<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let initialized = user::Entity::find()
        .count(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        > 0;
    let res = if initialized {
        info!("setup already initialized. aborting.");
        StatusCode::FORBIDDEN.into_response()
    } else {
        debug!("setup not initialized.");
        next.run(req).await
    };
    Ok(res)
}

pub async fn simple_auth<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let initialized = user::Entity::find()
        .count(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        > 0;
    let res = if initialized {
        info!("setup already initialized. aborting.");
        StatusCode::FORBIDDEN.into_response()
    } else {
        debug!("setup not initialized.");
        next.run(req).await
    };
    Ok(res)
}
