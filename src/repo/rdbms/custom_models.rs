use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Debug, Serialize, FromQueryResult)]
pub struct CharacterAndClassName {
    id: i32,
    name: String,
    class: String,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct CharacterOverview {
    id: i32,
    name: String,
    experience: i32,
    level: i32,
    class: String,
    pub account_id: i32,
    guild: Option<String>,
    pub mainhand_id: Option<i32>,
    mainhand_name: Option<String>,
    pub mainhand_rarity: Option<String>,
    pub mainhand_color: Option<String>,
    pub offhand_id: Option<i32>,
    offhand_name: Option<String>,
    pub offhand_rarity: Option<String>,
    pub offhand_color: Option<String>,
    pub head_id: Option<i32>,
    head_name: Option<String>,
    pub head_rarity: Option<String>,
    pub head_color: Option<String>,
    pub chest_id: Option<i32>,
    chest_name: Option<String>,
    pub chest_rarity: Option<String>,
    pub chest_color: Option<String>,
    pub hands_id: Option<i32>,
    hands_name: Option<String>,
    pub hands_rarity: Option<String>,
    pub hands_color: Option<String>,
    pub legs_id: Option<i32>,
    legs_name: Option<String>,
    pub legs_rarity: Option<String>,
    pub legs_color: Option<String>,
    pub feet_id: Option<i32>,
    feet_name: Option<String>,
    pub feet_rarity: Option<String>,
    pub feet_color: Option<String>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct InventoryOverviewItem {
    pub id: i32,
    pub name: String,
    pub slot: String,
    pub rarity: String,
    pub color: Option<String>,
    pub can_equip: Option<bool>,
    pub damage_lower: Option<i32>,
    pub damage_upper: Option<i32>,
    pub armor_lower: Option<i32>,
    pub armor_upper: Option<i32>,
}
