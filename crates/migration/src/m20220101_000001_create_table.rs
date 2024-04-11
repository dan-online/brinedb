use sea_orm_migration::prelude::*;

use crate::Brine;

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
          .col(ColumnDef::new(Brine::Key).text().primary_key())
          .col(ColumnDef::new(Brine::Value).text())
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    Ok(())
  }
}
