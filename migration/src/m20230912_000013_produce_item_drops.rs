use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        
        //============//
        // DUMMY DATA //
        //============//
        db.execute_unprepared("-- sql
            CALL drop_weapon(1);
            CALL drop_weapon(1);
            CALL drop_weapon(1);
            CALL drop_weapon(1);
            CALL drop_weapon(2);
            CALL drop_weapon(2);
            CALL drop_weapon(2);
            CALL drop_weapon(2);
            CALL drop_weapon(3);
            CALL drop_weapon(3);
            CALL drop_weapon(3);
            CALL drop_weapon(3);
            CALL drop_weapon(4);
            CALL drop_weapon(4);
            CALL drop_weapon(4);
            CALL drop_weapon(4);
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("
            DELETE from item_piece;
        ").await?;

        Ok(())
    }
}