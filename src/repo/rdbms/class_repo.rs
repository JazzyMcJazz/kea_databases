use sea_orm::{DatabaseConnection, DbErr, EntityTrait, JsonValue};

use crate::entity::class;

pub struct ClassRepo;

impl ClassRepo {
    pub async fn all(db: &DatabaseConnection) -> Result<Vec<JsonValue>, DbErr> {
        let classes = class::Entity::find().into_json().all(db).await?;
        Ok(classes)
    }
}
