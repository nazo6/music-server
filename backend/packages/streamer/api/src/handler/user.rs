use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use common::AppState;
use entity::current::*;
use sea_orm::{prelude::*, Set};
use serde::Deserialize;
use tracing::log::info;

#[derive(Deserialize)]
pub struct AddLibraryQuery {
    name: String,
    password: String,
    library_name: String,
    library_path: String,
}
pub async fn add_library(
    State(state): State<AppState>,
    query: Query<AddLibraryQuery>,
) -> StatusCode {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(query.name.clone()))
        .one(&state.conn)
        .await
        .unwrap();

    match user {
        Some(user) => {
            if user.password == query.password {
                let res = library::Entity::insert(library::ActiveModel {
                    name: Set(query.library_name.clone()),
                    path: Set(query.library_path.clone()),
                    user_id: Set(user.id),
                    ..Default::default()
                })
                .exec(&state.conn)
                .await;

                info!("adding library: {:?}", res);

                match res {
                    Ok(_) => StatusCode::OK,
                    Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            } else {
                StatusCode::UNAUTHORIZED
            }
        }
        None => StatusCode::UNAUTHORIZED,
    }
}
