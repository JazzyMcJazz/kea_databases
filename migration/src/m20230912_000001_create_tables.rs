use sea_orm_migration::{prelude::*, sea_orm::{EnumIter, DeriveActiveEnum}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        //===============//
        // ACCOUNT TABLE //
        //===============//
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Account::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Account::Password).string().not_null())
                    .col(ColumnDef::new(Account::LastLogin).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await?;

        //=============//
        // CLASS TABLE //
        //=============//
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Class::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Class::Name).string().unique_key().not_null())
                    .to_owned(),
            )
            .await?;

        //=================//
        // CHARACTER TABLE //
        //=================//
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Character::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(Character::Experience).integer().default(0).not_null())
                    .col(ColumnDef::new(Character::AccountId).integer().not_null())
                    .col(ColumnDef::new(Character::ClassId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_character_account_id")
                            .from(Character::Table, Character::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_character_class_id")
                            .from(Character::Table, Character::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;   

        //===================//
        // CLASS_SKILL TABLE //
        //===================//
        manager
            .create_table(
                Table::create()
                    .table(ClassSkill::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ClassSkill::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ClassSkill::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(ClassSkill::DamageUpper).integer().not_null())
                    .col(ColumnDef::new(ClassSkill::DamageLower).integer().not_null())
                    .col(ColumnDef::new(ClassSkill::ClassId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_class_skill_class_id")
                            .from(ClassSkill::Table, ClassSkill::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        //=================//
        // INVENTORY TABLE //
        //=================//
        manager
            .create_table(
                Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Inventory::CharacterId).integer().primary_key().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_character_id")
                            .from(Inventory::Table, Inventory::CharacterId)
                            .to(Character::Table, Character::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        //=============//
        // ITEM TABLE //
        //=============//
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(Item::Slot).enumeration(SlotEnum, Slot::iden_values()).not_null())
                    .col(ColumnDef::new(Item::Rarity).enumeration(RarityEnum, Rarity::iden_values()).not_null())
                    .to_owned(),
            )
            .await?;

        //====================//
        // WEAPON_SKILL TABLE //
        //====================//
        manager
            .create_table(
                Table::create()
                    .table(WeaponSkill::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeaponSkill::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WeaponSkill::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(WeaponSkill::DamageUpper).integer().not_null())
                    .col(ColumnDef::new(WeaponSkill::DamageLower).integer().not_null())
                    .col(ColumnDef::new(WeaponSkill::ItemId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_weapon_skill_item_id")
                            .from(WeaponSkill::Table, WeaponSkill::ItemId)
                            .to(Item::Table, Item::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        //==================//
        // ITEM_PIECE TABLE //
        //==================//
        manager
            .create_table(
                Table::create()
                    .table(ItemPiece::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ItemPiece::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemPiece::DamageLower).integer())
                    .col(ColumnDef::new(ItemPiece::DamageUpper).integer())
                    .col(ColumnDef::new(ItemPiece::ArmorLower).integer())
                    .col(ColumnDef::new(ItemPiece::ArmorUpper).integer())
                    .col(ColumnDef::new(ItemPiece::CharacterId).integer())
                    .col(ColumnDef::new(ItemPiece::InventoryId).integer())
                    .col(ColumnDef::new(ItemPiece::ItemId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_piece_character_id")
                            .from(ItemPiece::Table, ItemPiece::CharacterId)
                            .to(Character::Table, Character::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_piece_inventory_id")
                            .from(ItemPiece::Table, ItemPiece::InventoryId)
                            .to(Inventory::Table, Inventory::CharacterId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_item_piece_item_id")
                            .from(ItemPiece::Table, ItemPiece::ItemId)
                            .to(Item::Table, Item::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        //=============//
        // GUILD TABLE //
        //=============//
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Guild::Name).string().unique_key().not_null())
                    .to_owned(),
            )
            .await?;

        //====================//
        // GUILD_MEMBER TABLE //
        //====================//
        manager
            .create_table(
                Table::create()
                    .table(GuildMember::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GuildMember::AccountId).integer().primary_key().not_null())
                    .col(ColumnDef::new(GuildMember::GuildId).integer().not_null())
                    .col(ColumnDef::new(GuildMember::Role).enumeration(GuildRoleEnum, GuildRole::iden_values()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_member_character_id")
                            .from(GuildMember::Table, GuildMember::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_member_guild_id")
                            .from(GuildMember::Table, GuildMember::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // DROP ITEM_PIECE TABLE
        manager
            .drop_table(Table::drop().table(ItemPiece::Table).if_exists().to_owned())
            .await?;

        // DROP WEAPON_SKILL TABLE
        manager
            .drop_table(Table::drop().table(WeaponSkill::Table).if_exists().to_owned())
            .await?;

        // DROP ITEM TABLE
        manager
            .drop_table(Table::drop().table(Item::Table).if_exists().to_owned())
            .await?;

        // DROP INVENTORY TABLE
        manager
            .drop_table(Table::drop().table(Inventory::Table).if_exists().to_owned())
            .await?;

        // DROP CLASS_SKILL TABLE
        manager
        .drop_table(Table::drop().table(ClassSkill::Table).if_exists().to_owned())
        .await?;

        // DROP CHARACTER TABLE
        manager
        .drop_table(Table::drop().table(Character::Table).if_exists().to_owned())
        .await?;

        // DROP CLASS TABLE
        manager
            .drop_table(Table::drop().table(Class::Table).if_exists().to_owned())
            .await?;

        // DROP GUILD_MEMBER TABLE
        manager
            .drop_table(Table::drop().table(GuildMember::Table).if_exists().to_owned())
            .await?;

        // DROP GUILD TABLE
        manager
            .drop_table(Table::drop().table(Guild::Table).if_exists().to_owned())
            .await?;

        // DROP ACCOUNT TABLE
        manager
            .drop_table(Table::drop().table(Account::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Username,
    Password,
    LastLogin,
}

#[derive(DeriveIden)]
enum Character {
    Table,
    Id,
    Name,
    Experience,
    AccountId,
    ClassId,
}

#[derive(DeriveIden)]
enum Class {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum ClassSkill {
    Table,
    Id,
    Name,
    DamageUpper,
    DamageLower,
    ClassId,
}

#[derive(DeriveIden)]
enum Inventory {
    Table,
    CharacterId,
}

#[derive(DeriveIden)]
enum ItemPiece {
    Table,
    Id,
    DamageUpper,
    DamageLower,
    ArmorUpper,
    ArmorLower,
    CharacterId,
    InventoryId,
    ItemId,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    Name,
    Slot,
    Rarity,
}

#[derive(DeriveIden)]
enum WeaponSkill {
    Table,
    Id,
    Name,
    DamageUpper,
    DamageLower,
    ItemId,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum GuildMember {
    Table,
    AccountId,
    GuildId,
    Role,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "guild_role")]
enum GuildRole {
    #[sea_orm(string_value = "Member")]
    Member,
    #[sea_orm(string_value = "Leader")]
    Leader,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "rarity")]
enum Rarity {
    #[sea_orm(string_value = "Common")]
    Common,
    #[sea_orm(string_value = "Rare")]
    Rare,
    #[sea_orm(string_value = "Epic")]
    Epic,
    #[sea_orm(string_value = "Legendary")]
    Legendary,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "slot")]
enum Slot {
    #[sea_orm(string_value = "Head")]
    Head,
    #[sea_orm(string_value = "Chest")]
    Chest,
    #[sea_orm(string_value = "Hands")]
    Hands,
    #[sea_orm(string_value = "Legs")]
    Legs,
    #[sea_orm(string_value = "Feet")]
    Feet,
    #[sea_orm(string_value = "MainHand")]
    MainHand,
    #[sea_orm(string_value = "OffHand")]
    OffHand,
}

