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
            INSERT INTO account (username, password) VALUES ('Bob', '123456');
            INSERT INTO account (username, password) VALUES ('John', '123456');
            INSERT INTO account (username, password) VALUES ('Jane', '123456');
            INSERT INTO account (username, password) VALUES ('Oledinho', '123456');

            INSERT INTO `character` (name, account_id, class_id) VALUES ('Bobby', 1, 1);
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