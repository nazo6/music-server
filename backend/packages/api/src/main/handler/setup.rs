use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use common::AppState;
use entity::current::*;
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use tracing::log::info;

#[derive(Deserialize)]
pub struct SetAdminQuery {
    name: String,
    password: String,
}
pub async fn set_admin(State(state): State<AppState>, query: Query<SetAdminQuery>) -> StatusCode {
    let res = user::Entity::insert(user::ActiveModel {
        name: Set(query.name.clone()),
        password: Set(query.password.clone()),
        is_admin: Set(true),
        ..Default::default()
    })
    .exec(&state.conn)
    .await;

    info!("creating admin user: {:?}", res);

    match res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
