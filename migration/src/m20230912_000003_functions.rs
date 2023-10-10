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
            -- Returns damage/armor multiplier based on rarity
            DROP FUNCTION IF EXISTS GetMultiplier;
            CREATE FUNCTION GetMultiplier(
                rarity ENUM('Common', 'Rare', 'Epic', 'Legendary')
            )
            RETURNS INT(1)
            BEGIN
                CASE
                    WHEN rarity = 'Rare' THEN RETURN 2;
                    WHEN rarity = 'Epic' THEN RETURN 3;
                    WHEN rarity = 'Legendary' THEN RETURN 4;
                    ELSE RETURN 1;
                END CASE;
            END;
        ").await?;


        db.execute_unprepared("-- sql

            -- Returns character level based on experience
            DROP FUNCTION IF EXISTS GetCharacterLevel;
            CREATE FUNCTION GetCharacterLevel(
                total_xp INT(11)
            )
            RETURNS INT(11)
            BEGIN
                DECLARE level INT DEFAULT 1;
                DECLARE xp_required INT DEFAULT 0;
                DECLARE i INT DEFAULT 1;

                WHILE level <= 20 DO
                    SET xp_required = 0; -- Reset for each level

                    -- Calculate XP required for the current level
                    SET i = 1;
                    REPEAT
                        SET xp_required = xp_required + (i * i * 100);
                        SET i = i + 1;
                    UNTIL i > level
                    END REPEAT;

                    -- Check if the character's level is found
                    IF total_xp < xp_required THEN
                        RETURN level - 1;
                    END IF;

                    SET level = level + 1;
                END WHILE;

                -- If we've exited the loop, the character is level 20
                RETURN 20;
            END;
        ").await?;

        db.execute_unprepared("-- sql
            DROP FUNCTION IF EXISTS GetRarityColor;
            CREATE FUNCTION GetRarityColor(
                rarity ENUM('Common', 'Rare', 'Epic', 'Legendary')
            )
            RETURNS VARCHAR(7)
            BEGIN
                CASE
                    WHEN rarity = 'Common' THEN RETURN 'grey';
                    WHEN rarity = 'Rare' THEN RETURN 'skyblue';
                    WHEN rarity = 'Epic' THEN RETURN 'gold';
                    WHEN rarity = 'Legendary' THEN RETURN 'purple';
                    ELSE RETURN NULL;
                END CASE;
            END;
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("-- sql
            DROP FUNCTION IF EXISTS GetMultiplier;
            DROP FUNCTION IF EXISTS GetCharacterLevel;
            DROP FUNCTION IF EXISTS GetRarityColor;
        ").await?;

        Ok(())
    }
}