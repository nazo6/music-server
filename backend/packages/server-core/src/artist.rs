use entity::current::artist;
use sea_orm::prelude::*;

use crate::errors::Error;

pub async fn get_artists(library_id: i32) -> Result<Vec<artist::Model>, Error> {
    let db = common::get_db().await;
    let artists = artist::Entity::find()
        .filter(artist::Column::LibraryId.eq(library_id))
        .all(db)
        .await?;
    Ok(artists)
}
