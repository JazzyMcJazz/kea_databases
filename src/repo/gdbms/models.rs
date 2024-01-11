use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::utils::traits::Thingify;

use super::enums::{GuildRole, Rarity, Slot};

/////////////////////
// Database Models //
/////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub uid: Option<String>,
    pub id: Thing,
    pub username: String,
    pub password: String,
    last_login: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub uid: Option<String>,
    id: Thing,
    name: String,
    members: Vec<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMember {
    username: String,
    rank: String,
    role: GuildRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub uid: Option<String>,
    id: Thing,
    pub account_id: String,
    name: String,
    experience: u64,
    class: Class,
    pub equipped_gear: Vec<Gear>,
    pub inventory: Vec<Gear>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gear {
    pub id: Thing,
    pub name: String,              
    pub slot: Slot,                
    pub rarity: Rarity,           
    weapon_skill: Option<WeaponSkill>, // if weapon
    upper_stat: u32,         
    lower_stat: u32,         
    pub can_equip: Option<bool>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub uid: Option<String>,
    pub id: Option<Thing>,
    pub name: String,
    skills: Vec<ClassSkill>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassSkill {
    name: String,
    damage_upper: u32,
    damage_lower: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub uid: Option<String>,
    id: Thing,
    name: String,
    rarity: Rarity,
    slot: Slot,
    weapon_skill: Option<WeaponSkill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeaponSkill {
    name: String,
    damage_upper: u32,
    damage_lower: u32,
}

//////////////////////////////
// Thingify implementations //
//////////////////////////////

impl Thingify for Account {
    #[allow(dead_code)]
    fn thingify(&mut self) {
        self.uid = Some(self.id.id.to_raw());
    }
}
impl Thingify for Guild {
    #[allow(dead_code)]
    fn thingify(&mut self) {
        self.uid = Some(self.id.id.to_raw());
    }
}
impl Thingify for Character {
    #[allow(dead_code)]
    fn thingify(&mut self) {
        self.uid = Some(self.id.id.to_raw());
    }
}
impl Thingify for Class {
    #[allow(dead_code)]
    fn thingify(&mut self) {
        self.uid = Some(self.id.clone().unwrap().id.to_raw());
    }
}
impl Thingify for Item {
    #[allow(dead_code)]
    fn thingify(&mut self) {
        self.uid = Some(self.id.id.to_raw());
    }
}
