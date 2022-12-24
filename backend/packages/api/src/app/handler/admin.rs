use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use common::AppState;
use entity::current::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

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
        .await;
    match user {
        Ok(Some(user)) => {
            library::Entity::insert(library::ActiveModel {
                path: Set(query.library_path.clone()),
                name: Set(query.library_name.clone()),
                ..Default::default()
            });
            StatusCode::OK
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

type UserList = Vec<String>;
pub async fn get_user_list(State(state): State<AppState>) -> Json<UserList> {
    let users = user::Entity::find().all(&state.conn).await.unwrap();

    let result = users
        .iter()
        .map(|user| user.name.clone())
        .collect::<Vec<String>>();

    Json(result)
}
