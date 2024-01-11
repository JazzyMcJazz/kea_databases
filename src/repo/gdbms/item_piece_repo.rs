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
}