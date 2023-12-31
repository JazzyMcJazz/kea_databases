//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item_piece")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub damage_lower: Option<i32>,
    pub damage_upper: Option<i32>,
    pub armor_lower: Option<i32>,
    pub armor_upper: Option<i32>,
    pub character_id: Option<i32>,
    pub inventory_id: Option<i32>,
    pub item_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::CharacterId",
        to = "super::character::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Character,
    #[sea_orm(
        belongs_to = "super::inventory::Entity",
        from = "Column::InventoryId",
        to = "super::inventory::Column::CharacterId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Inventory,
    #[sea_orm(
        belongs_to = "super::item::Entity",
        from = "Column::ItemId",
        to = "super::item::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Item,
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
