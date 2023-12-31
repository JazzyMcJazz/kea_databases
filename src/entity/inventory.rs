//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub character_id: i32,
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
    #[sea_orm(has_many = "super::item_piece::Entity")]
    ItemPiece,
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Character.def()
    }
}

impl Related<super::item_piece::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemPiece.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
