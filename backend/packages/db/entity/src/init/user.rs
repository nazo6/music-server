use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        super::library_user_relation::Relation::Library.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::library_user_relation::Relation::User.def().rev())
    }
}
