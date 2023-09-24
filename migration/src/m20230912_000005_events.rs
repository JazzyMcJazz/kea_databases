use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        
        //========//
        // EVENTS //
        //========//
        db.execute_unprepared("-- sql 
            DROP EVENT IF EXISTS e_delete_inactive;
            CREATE EVENT e_delete_inactive
            ON SCHEDULE EVERY '1 00' DAY_HOUR
            DO BEGIN
                DELETE FROM account WHERE last_login < NOW() - INTERVAL 1 YEAR;
            END;
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("-- sql
            DROP EVENT IF EXISTS e_delete_inactive;
        ").await?;

        Ok(())
    }
}