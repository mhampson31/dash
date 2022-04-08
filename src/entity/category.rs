//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::service_categories::Entity")]
    ServiceCategories,
    #[sea_orm(has_many = "super::user_categories::Entity")]
    UserCategories,
}

impl Related<super::service_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceCategories.def()
    }
}

impl Related<super::user_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
