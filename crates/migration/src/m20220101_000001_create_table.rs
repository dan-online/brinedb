use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Brine::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Brine::Key).string().not_null().primary_key())
                    .col(ColumnDef::new(Brine::Value).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Brine {
    Table,
    Key,
    Value,
}
