use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        
        //===================//
        // STORED PROCEDURES //
        //===================//
        db.execute_unprepared("-- sql 
            -- Drops a random item into the inventory of the character
            DROP PROCEDURE IF EXISTS drop_weapon;
            CREATE PROCEDURE drop_weapon(
                IN char_id INT(11)
            )
            BEGIN
                DECLARE new_item_id INT(11);
                DECLARE item_slot ENUM('Head', 'Chest', 'Hands', 'Legs', 'Feet', 'MainHand', 'OffHand');
                DECLARE rarity VARCHAR(30);
                DECLARE dmg_upper INT(9);
                DECLARE dmg_lower INT(9);
                DECLARE def_upper INT(9);
                DECLARE def_lower INT(9);
            
                IF char_id IS NULL THEN
                    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'character_id cannot be null';
                END IF;
            
                -- Select random item
                SELECT i.id, i.slot, i.rarity
                INTO new_item_id, item_slot, rarity
                FROM item i ORDER BY RAND() LIMIT 1;

                IF item_slot = 'MainHand' OR item_slot = 'OffHand' THEN
                    SET dmg_lower = FLOOR(50 + (100 - 50) * RAND()) * GetMultiplier(rarity);
                    SET dmg_upper = FLOOR(101 + (150 - 49) * RAND()) * GetMultiplier(rarity);
                    SET def_lower = NULL;
                    SET def_upper = NULL;
                ELSE
                    SET def_lower = FLOOR(50 + (100 - 50) * RAND()) * GetMultiplier(rarity);
                    SET def_upper = FLOOR(101 + (150 - 49) * RAND()) * GetMultiplier(rarity);
                    SET dmg_lower = NULL;
                    SET dmg_upper = NULL;
                END IF;
                INSERT INTO item_piece (damage_lower, damage_upper, armor_lower, armor_upper, character_id, inventory_id, item_id)
                VALUES (dmg_lower, dmg_upper, def_lower, def_upper, NULL, char_id, new_item_id);
            END;
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("-- sql
            DROP PROCEDURE IF EXISTS drop_weapon;
            DROP FUNCTION IF EXISTS GetMultiplier;
        ").await?;

        Ok(())
    }
}