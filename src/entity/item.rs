//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use super::sea_orm_active_enums::Rarity;
use super::sea_orm_active_enums::Slot;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub slot: Slot,
    pub rarity: Rarity,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::item_piece::Entity")]
    ItemPiece,
    #[sea_orm(has_many = "super::weapon_skill::Entity")]
    WeaponSkill,
}

impl Related<super::item_piece::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemPiece.def()
    }
}

impl Related<super::weapon_skill::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WeaponSkill.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
