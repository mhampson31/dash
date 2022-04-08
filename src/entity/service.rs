use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "service")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::service_categories::Entity")]
    ServiceCategories,
}

impl Related<super::service_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
