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
            INSERT INTO account (username, password) VALUES ('Bob', '$2b$12$iHEDiUbrX.MMbSea8GeRbOxZgFbYv7l5XB8BnLRW23B4baYVPIK96');
            INSERT INTO account (username, password) VALUES ('John', '$2b$12$iHEDiUbrX.MMbSea8GeRbOxZgFbYv7l5XB8BnLRW23B4baYVPIK96');
            INSERT INTO account (username, password) VALUES ('Jane', '$2b$12$iHEDiUbrX.MMbSea8GeRbOxZgFbYv7l5XB8BnLRW23B4baYVPIK96');
            INSERT INTO account (username, password) VALUES ('Oledinho', '$2b$12$iHEDiUbrX.MMbSea8GeRbOxZgFbYv7l5XB8BnLRW23B4baYVPIK96');

            INSERT INTO `character` (name, account_id, class_id) VALUES ('Bobby', 1, 1);
            INSERT INTO `character` (name, account_id, class_id) VALUES ('Bonnie', 1, 2);
            INSERT INTO `character` (name, account_id, class_id) VALUES ('Johnny', 2, 2);
            INSERT INTO `character` (name, account_id, class_id) VALUES ('Janey', 3, 3);
            INSERT INTO `character` (name, account_id, class_id) VALUES ('Ole', 4, 4);
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("
            DELETE from account;
        ").await?;

        Ok(())
    }
}