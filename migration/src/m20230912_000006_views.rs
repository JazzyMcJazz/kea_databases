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
            CREATE or REPLACE VIEW character_overview AS
            SELECT
                c.id,
                c.name,
                GetCharacterLevel(experience) AS level,
                class.name AS class,
                c.account_id,
                IFNULL(guild.name, NULL) AS guild,
                mainhand_item.name AS mainhand,
                offhand_item.name AS offhand,
                head_item.name AS head,
                chest_item.name AS chest,
                hands_item.name AS hands,
                legs_item.name AS legs,
                feet_item.name AS feet
            FROM `character` c
            JOIN class ON class.id = c.class_id
            LEFT JOIN guild_member gm ON gm.account_id = c.account_id
            LEFT JOIN guild ON guild.id = gm.guild_id
            LEFT JOIN item_piece mainhand_piece ON mainhand_piece.character_id = c.id
            LEFT JOIN item mainhand_item ON mainhand_piece.item_id = mainhand_item.id AND mainhand_item.slot = 'MainHand'
            LEFT JOIN item_piece offhand_piece ON offhand_piece.character_id = c.id
            LEFT JOIN item offhand_item ON offhand_piece.item_id = offhand_item.id AND offhand_item.slot = 'OffHand'
            LEFT JOIN item_piece head_piece ON head_piece.character_id = c.id
            LEFT JOIN item head_item ON head_piece.item_id = head_item.id AND head_item.slot = 'Head'
            LEFT JOIN item_piece chest_piece ON chest_piece.character_id = c.id
            LEFT JOIN item chest_item ON chest_piece.item_id = chest_item.id AND chest_item.slot = 'Chest'
            LEFT JOIN item_piece hands_piece ON hands_piece.character_id = c.id
            LEFT JOIN item hands_item ON hands_piece.item_id = hands_item.id AND hands_item.slot = 'Hands'
            LEFT JOIN item_piece legs_piece ON legs_piece.character_id = c.id
            LEFT JOIN item legs_item ON legs_piece.item_id = legs_item.id AND legs_item.slot = 'Legs'
            LEFT JOIN item_piece feet_piece ON feet_piece.character_id = c.id
            LEFT JOIN item feet_item ON feet_piece.item_id = feet_item.id AND feet_item.slot = 'Feet';
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