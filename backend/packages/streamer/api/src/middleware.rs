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
) -> Response {
    let initialized = user::Entity::find()
        .paginate(&state.conn, 1)
        .num_items()
        .await
        .unwrap()
        > 0;
    if initialized {
        info!("setup already initialized. aborting.");
        StatusCode::FORBIDDEN.into_response()
    } else {
        debug!("setup not initialized.");
        next.run(req).await
    }
}
