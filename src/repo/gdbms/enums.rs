use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum GuildRole {
    Leader,
    Member,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub enum Slot {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}
