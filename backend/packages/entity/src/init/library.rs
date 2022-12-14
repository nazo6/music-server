use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "libraries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Id",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::track::Entity")]
    Track,
    #[sea_orm(has_many = "super::artist::Entity")]
    Artist,
    #[sea_orm(has_many = "super::album::Entity")]
    Album,
    #[sea_orm(has_many = "super::genre::Entity")]
    Genre,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
