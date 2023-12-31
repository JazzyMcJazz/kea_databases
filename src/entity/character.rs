//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub experience: i32,
    pub account_id: i32,
    pub class_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::class::Entity",
        from = "Column::ClassId",
        to = "super::class::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Class,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(has_many = "super::item_piece::Entity")]
    ItemPiece,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::item_piece::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemPiece.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
