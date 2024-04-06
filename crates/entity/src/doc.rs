use sea_orm::entity::prelude::*;
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "brine")]
pub struct Model {
    #[sea_orm(primary_key, indexed, unique)]
    pub key: String,
    #[sea_orm(nullable)]
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
