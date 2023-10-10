use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        
        //========//
        // VIEWS //
        //========//
        db.execute_unprepared("-- sql 
        CREATE OR REPLACE VIEW character_overview AS
        SELECT
            c.id,
            c.name,
            c.experience,
            GetCharacterLevel(experience) AS level,
            class.name AS class,
            c.account_id,
            IFNULL(guild.name, NULL) AS guild,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'MainHand') AS mainhand_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'MainHand') AS mainhand_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'MainHand') AS mainhand_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'MainHand')) AS mainhand_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'OffHand') AS offhand_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'OffHand') AS offhand_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'OffHand') AS offhand_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'OffHand')) AS offhand_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Head') AS head_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Head') AS head_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Head') AS head_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Head')) AS head_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Chest') AS chest_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Chest') AS chest_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Chest') AS chest_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Chest')) AS chest_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Hands') AS hands_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Hands') AS hands_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Hands') AS hands_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Hands')) AS hands_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Legs') AS legs_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Legs') AS legs_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Legs') AS legs_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Legs')) AS legs_color,
            (SELECT item.id FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Feet') AS feet_id,
            (SELECT item.name FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Feet') AS feet_name,
            (SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Feet') AS feet_rarity,
            GetRarityColor((SELECT item.rarity FROM item_piece JOIN item ON item_piece.item_id = item.id WHERE item_piece.character_id = c.id AND item.slot = 'Feet')) AS feet_color
        FROM `character` c
        JOIN class ON class.id = c.class_id
        LEFT JOIN guild_member gm ON gm.account_id = c.account_id
        LEFT JOIN guild ON guild.id = gm.guild_id;
        ").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db = manager.get_connection();

        db.execute_unprepared("-- sql
            DROP VIEW IF EXISTS character_overview;
        ").await?;

        Ok(())
    }
}