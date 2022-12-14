use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "album_artist")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub album_id: u64,
    pub artist_id: u64,
}
impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Artist,
    Album,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Album => Entity::belongs_to(super::album::Entity)
                .from(Column::AlbumId)
                .to(super::album::Column::Id)
                .into(),
            Self::Artist => Entity::belongs_to(super::artist::Entity)
                .from(Column::ArtistId)
                .to(super::artist::Column::Id)
                .into(),
        }
    }
}
