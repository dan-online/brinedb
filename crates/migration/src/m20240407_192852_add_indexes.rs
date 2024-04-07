use sea_orm_migration::prelude::*;

use crate::Brine;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(Brine::Table)
                    .col(Brine::Key)
                    .name("idx_key")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().table(Brine::Table).name("idx_key").to_owned())
            .await
    }
}
