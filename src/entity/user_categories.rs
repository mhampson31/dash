//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub person_id: i32,
    #[sea_orm(primary_key)]
    pub category_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::PersonId",
        to = "super::person::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Person,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}