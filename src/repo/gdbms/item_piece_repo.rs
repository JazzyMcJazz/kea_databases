use surrealdb::{Surreal, Error, engine::any::Any};



pub struct ItemPieceRepo;

impl ItemPieceRepo {
    pub async fn fn_drop_loot(db: &Surreal<Any>, character_id: &String) -> Result<(), Error> {
        db
            .query("fn::drop_item($character_id);")
            .bind(("character_id", character_id))
            .await?;
        
        Ok(())
    }

    pub async fn equip_by_id(db: &Surreal<Any>, character_id: &String, item_id: &String) -> Result<(), Error> {
        db
            .query(r#"
                BEGIN TRANSACTION;

                LET $character = type::thing("character", $character_id);
                LET $item_piece = type::thing("item_piece", $item_id);

                LET $relation = SELECT * FROM unequipped WHERE in=$character AND out=$item_piece;
                
                IF (!$relation) {
                    THROW "Character does not have that item in their inventory";
                };
                
                DELETE $character->unequipped WHERE out=$item_piece;
                RELATE $character->equipped->$item_piece;
                
                COMMIT TRANSACTION;
            "#)
            .bind(("character_id", character_id))
            .bind(("item_id", item_id))
            .await?;
        
        Ok(())
    }

    pub async fn unequip_by_id(db: &Surreal<Any>, character_id: &String, item_id: &String) -> Result<(), Error> {
        db
            .query(r#"
                BEGIN TRANSACTION;

                LET $character = type::thing("character", $character_id);
                LET $item_piece = type::thing("item_piece", $item_id);

                LET $relation = SELECT * FROM equipped WHERE in=$character AND out=$item_piece;
                
                IF (!$relation) {
                    THROW "Character does not have that item equipped";
                };

                LET $character = type::thing("character", $character_id);
                LET $item_piece = type::thing("item_piece", $item_id);
                
                DELETE $character->equipped WHERE out=$item_piece;
                RELATE $character->unequipped->$item_piece;
                
                COMMIT TRANSACTION;
            "#)
            .bind(("character_id", character_id))
            .bind(("item_id", item_id))
            .await?;
        
        Ok(())
    }

    pub async fn delete_by_id(db: &Surreal<Any>, character_id: &String, item_id: &String) -> Result<(), Error> {
        db
            .query(r#"
                LET $character = type::thing("character", $character_id);
                LET $item_piece = type::thing("item_piece", $item_id);
                LET $equipped   = SELECT * FROM equipped   WHERE in=$character AND out=$item_piece;
                LET $unequipped = SELECT * FROM unequipped WHERE in=$character AND out=$item_piece;

                IF (!$equipped AND !$unequipped) {
                    THROW "Character does not have that item";
                };

                DELETE $item_piece;
            "#)
            .bind(("character_id", character_id))
            .bind(("item_id", item_id))
            .await?;
        
        Ok(())
    }
}