use entity::current::album;
use sea_orm::prelude::*;

use crate::errors::Error;

pub async fn get_albums(library_id: i32) -> Result<Vec<album::Model>, Error> {
    let db = common::get_db().await;
    let albums = album::Entity::find()
        .filter(album::Column::LibraryId.eq(library_id))
        .all(db)
        .await?;
    Ok(albums)
}
