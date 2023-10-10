use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, JoinType, QuerySelect, RelationTrait};
use crate::entity::{item_piece::{Entity as ItemPiece, self}, item};

use super::custom_models::InventoryOverviewItem;


pub struct InventoryRepo;

impl InventoryRepo {
    pub async fn find_item_pieces_by_inventory_id(conn: &DatabaseConnection, id: i32) -> Result<Vec<InventoryOverviewItem>, DbErr> {
        let mut inventory = ItemPiece::find()
            .filter(item_piece::Column::InventoryId.eq(id))
            .column_as(item::Column::Name, "name")
            .column_as(item::Column::Slot, "slot")
            .column_as(item::Column::Rarity, "rarity")
            .join(JoinType::Join, item_piece::Relation::Item.def(),
            )
            .into_model::<InventoryOverviewItem>()
            .all(conn)
            .await?;

        for item in inventory.iter_mut() {
            item.color = match item.rarity.as_str() {
                "Common" => Some("grey".to_string()),
                "Rare" => Some("skyblue".to_string()),
                "Epic" => Some("gold".to_string()),
                "Legendary" => Some("purple".to_string()),
                _ => None,
            }
        }

        Ok(inventory)
    }
}