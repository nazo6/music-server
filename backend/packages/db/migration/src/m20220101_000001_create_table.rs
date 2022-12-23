use entity::*;
use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(init::user::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::library::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::genre::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::album::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::artist::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::track::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::artist_track_relation::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::library_user_relation::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(init::album_artist_relation::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(init::artist_track_relation::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(init::album_artist_relation::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(init::library_user_relation::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(init::artist::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(init::track::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(init::album::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(init::genre::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(init::library::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(init::user::Entity).to_owned())
            .await
    }
}
