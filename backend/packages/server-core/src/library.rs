use common::get_db;
use entity::current::*;
use sea_orm::{prelude::*, Set, TransactionTrait};
use tracing::instrument;

use crate::errors::Error;

/// create new library
#[instrument]
pub async fn create_library(name: &str, path: &str) -> Result<library::Model, Error> {
    let txn = get_db().await.begin().await?;

    let library = library::ActiveModel {
        name: Set(name.to_string()),
        path: Set(path.to_string()),
        ..Default::default()
    }
    .insert(get_db().await)
    .await?;

    txn.commit().await?;

    Ok(library)
}
