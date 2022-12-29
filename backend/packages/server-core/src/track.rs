use entity::current::track;
use sea_orm::prelude::*;

use crate::errors::Error;

pub async fn get_tracks(library_id: i32) -> Result<Vec<track::Model>, Error> {
    let db = common::get_db().await;
    let tracks = track::Entity::find()
        .filter(track::Column::LibraryId.eq(library_id))
        .all(db)
        .await?;
    Ok(tracks)
}
