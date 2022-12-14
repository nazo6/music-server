//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "track")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub duration: String,
    pub created_at: String,
    pub star: Option<i16>,
    pub played_count: Uuid,
    pub file_path: String,
    pub library_id: Uuid,
    pub album_id: Option<Uuid>,
    pub genre_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Album,
    #[sea_orm(
        belongs_to = "super::genre::Entity",
        from = "Column::GenreId",
        to = "super::genre::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genre,
    #[sea_orm(
        belongs_to = "super::library::Entity",
        from = "Column::LibraryId",
        to = "super::library::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Library,
}

impl Related<super::album::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Album.def()
    }
}

impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genre.def()
    }
}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Library.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        super::artist_track_relation::Relation::Artist.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::artist_track_relation::Relation::Track.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
