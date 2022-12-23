use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "track")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub duration: String,
    pub created_at: String,
    pub star: Option<u8>,
    pub played_count: i32,
    pub file_path: String,
    pub library_id: i32,
    pub album_id: Option<i32>,
    pub genre_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::library::Entity",
        from = "Column::LibraryId",
        to = "super::library::Column::Id"
    )]
    Library,
    #[sea_orm(
        belongs_to = "super::album::Entity",
        from = "Column::AlbumId",
        to = "super::album::Column::Id"
    )]
    Album,
    #[sea_orm(
        belongs_to = "super::genre::Entity",
        from = "Column::GenreId",
        to = "super::genre::Column::Id"
    )]
    Genre,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Library.def()
    }
}
impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genre.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_artist_relation::Relation::Artist.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_artist_relation::Relation::Album.def().rev())
    }
}
