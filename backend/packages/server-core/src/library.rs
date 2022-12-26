use common::get_db;
use entity::current::*;
use sea_orm::{prelude::*, Set, TransactionTrait};
use tracing::instrument;

use crate::errors::Error;

/// create new library
#[instrument]
pub async fn create_library(name: &str, path: &str) -> Result<library::Model, Error> {
    let library = library::ActiveModel {
        name: Set(name.to_string()),
        path: Set(path.to_string()),
        ..Default::default()
    }
    .insert(get_db().await)
    .await?;

    Ok(library)
}

/// return library info by id
#[instrument]
pub async fn get_library(id: i32) -> Result<Option<library::Model>, Error> {
    Ok(library::Entity::find_by_id(id).one(get_db().await).await?)
}
