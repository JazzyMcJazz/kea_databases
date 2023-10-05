use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter, QuerySelect, RelationTrait, JoinType, Statement, DbBackend, Set};

use crate::{entity::{character::{Entity as Character, self}, class}, repo::rdbms::custom_models::CharacterOverview};

use super::custom_models::CharacterAndClassName;

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn get_by_account_id(db: &DatabaseConnection, id: i32) -> Result<Vec<CharacterAndClassName>, DbErr> {
        let characters = Character::find()
            .filter(character::Column::AccountId.eq(id))
            .column_as(class::Column::Name, "class")
            .join(
                JoinType::Join, 
                character::Relation::Class.def(),
            )
            .into_model::<CharacterAndClassName>()
            .all(db)
            .await?;

        Ok(characters)
    }

    pub async fn get_view_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<CharacterOverview>, DbErr> {
        let character = Character::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::MySql, 
                r#"-- sql
                SELECT * FROM character_overview WHERE id = ?;
            "#, 
                [id.into()]
            ))
            .into_model::<CharacterOverview>()
            .one(db)
            .await?;

        println!("{:#?}", character);

        Ok(character)
    }

    pub async fn create(db: &DatabaseConnection, name: &str, class_id: i32, account_id: i32) -> Result<i32, DbErr> {
        let result = Character::insert(
            character::ActiveModel {
                name: Set(name.to_owned()),
                class_id: Set(class_id),
                account_id: Set(account_id),
                ..Default::default()
            }
        )
        .exec(db)
        .await?;

        Ok(result.last_insert_id)
    }
}