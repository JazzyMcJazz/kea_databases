use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Set, Statement,
    Value,
};

use crate::entity::{character, item_piece, prelude::ItemPiece};

pub struct ItemPieceRepo;

impl ItemPieceRepo {
    pub async fn sp_drop_loot(
        db: &DatabaseConnection,
        inventory_id: i32,
    ) -> Result<(), sea_orm::DbErr> {
        db.execute(Statement::from_sql_and_values(
            DbBackend::MySql,
            "CALL drop_weapon(?)",
            IntoIterator::into_iter(vec![Value::from(inventory_id)]),
        ))
        .await?;

        let character = character::Entity::find_by_id(inventory_id).one(db).await?;

        let mut character: character::ActiveModel = character.unwrap().into();
        character.experience = Set(character.experience.unwrap() + 100);
        character.update(db).await?;

        Ok(())
    }

    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<(), sea_orm::DbErr> {
        ItemPiece::delete_by_id(id).exec(db).await?;

        Ok(())
    }

    pub async fn equip_by_id(
        db: &DatabaseConnection,
        character_id: i32,
        id: i32,
    ) -> Result<(), sea_orm::DbErr> {
        let model = item_piece::ActiveModel {
            id: Set(id),
            inventory_id: Set(None),
            character_id: Set(Some(character_id)),
            ..Default::default()
        };

        let _ = model.update(db).await;

        Ok(())
    }
}
