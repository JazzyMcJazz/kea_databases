use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        
        //===================//
        // CHARACTER TRIGGER //
        //===================//
        db.execute_unprepared("-- sql
            CREATE OR REPLACE TRIGGER character_after_insert
            AFTER INSERT ON `character`
                FOR EACH ROW
                BEGIN
                    INSERT INTO inventory VALUES (NEW.id);
                END;
        ").await?;

        //=======================//
        // WEAPON_SKILL TRIGGERS //
        //=======================//
        db.execute_unprepared("-- sql
            CREATE OR REPLACE TRIGGER weapon_skill_before_insert
            BEFORE INSERT ON weapon_skill
                FOR EACH ROW
                BEGIN
                    DECLARE slot_type ENUM('MainHand', 'OffHand');
                    SELECT slot FROM item WHERE id = NEW.item_id INTO slot_type;
                END;

            CREATE OR REPLACE TRIGGER weapon_skill_before_update
            BEFORE UPDATE ON weapon_skill
                FOR EACH ROW
                BEGIN
                    DECLARE slot_type ENUM('MainHand', 'OffHand');
                    SELECT slot FROM item WHERE id = NEW.item_id INTO slot_type;
                END;
        ").await?;
        // NOTE: Just selecting into slot_type is enough to throw an error if the item is not a weapon

        //=======================//
        // ITEM_PIECE TRIGGERS //
        //=======================//
        let item_piece_triggers = "-- sql
            FOR EACH ROW
            BEGIN
                DECLARE slot_occupied TINYINT;
                DECLARE item_slot ENUM('Head', 'Chest', 'Hands', 'Legs', 'Feet', 'MainHand', 'OffHand');
                -- Ensure that exactly one of character_id and inventory_id is not null
                IF
                    NEW.character_id IS NOT NULL AND NEW.inventory_id IS NOT NULL OR
                    NEW.character_id IS NULL AND NEW.inventory_id IS NULL THEN
                        SIGNAL SQLSTATE '45000'
                            SET MESSAGE_TEXT = 'Item piece must belong to a character or inventory';
                END IF;
                SELECT slot INTO item_slot
                FROM item WHERE id = NEW.item_id;

                IF item_slot IS NULL THEN
                    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'item_slot is NULL';
                END IF;
                SELECT EXISTS(
                    SELECT * FROM item_piece
                    JOIN item i
                    WHERE character_id = NEW.character_id AND
                    i.slot = item_slot
                ) INTO slot_occupied;
        
                IF slot_occupied THEN
                    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Gear slot is already occupied';
                END IF;
        
                IF item_slot = 'MainHand' OR item_slot = 'OffHand' THEN
                    IF NEW.damage_lower IS NULL OR
                    NEW.damage_upper IS NULL OR
                    NEW.damage_lower > NEW.damage_upper OR
                    NEW.armor_lower IS NOT NULL OR
                    NEW.armor_upper IS NOT NULL THEN
                            SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'A weapon must only have damage values';
                    END IF;
                ELSE
                    IF NEW.armor_lower IS NULL OR
                    NEW.armor_upper IS NULL OR
                    NEW.armor_lower > NEW.armor_upper OR
                    NEW.damage_lower IS NOT NULL OR
                    NEW.damage_upper IS NOT NULL THEN
                            SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Armor must only have armor values';
                    END IF;
                END IF;
            END;
        ";
        
        db.execute_unprepared(format!("-- sql
            CREATE OR REPLACE TRIGGER item_piece_before_insert
            BEFORE INSERT ON item_piece
            {}
            
            CREATE OR REPLACE TRIGGER item_piece_before_update
            BEFORE UPDATE ON item_piece
            {}    
        ", item_piece_triggers, item_piece_triggers).as_str()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("-- sql
            DROP TRIGGER IF EXISTS character_after_insert;
            DROP TRIGGER IF EXISTS weapon_skill_before_insert;
            DROP TRIGGER IF EXISTS weapon_skill_before_update;
            DROP TRIGGER IF EXISTS item_piece_before_insert;
            DROP TRIGGER IF EXISTS item_piece_before_update;
        ").await?;

        Ok(())
    }
}