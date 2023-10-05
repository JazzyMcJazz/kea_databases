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
    level: i32,
    class: String,
    pub account_id: i32,
    guild: Option<String>,
    mainhand: Option<String>,
    offhand: Option<String>,
    head: Option<String>,
    chest: Option<String>,
    hands: Option<String>,
    legs: Option<String>,
    feet: Option<String>,
}