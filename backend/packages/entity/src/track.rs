use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub title: String,
    pub duration: String,
    pub created_at: String,
    pub star: Option<u8>,
    pub played_count: u64,
    pub file_path: String,
    pub album: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::library::Entity",
        from = "Column::Id",
        to = "super::library::Column::Id"
    )]
    Library,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Library.def()
    }
}
