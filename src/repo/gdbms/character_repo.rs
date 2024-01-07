use std::collections::HashMap;

use surrealdb::{engine::any::Any, sql::Thing, Error, Surreal};

use super::models::{Character, Class, Gear};

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn get_by_account_id(db: &Surreal<Any>, id: String) -> Result<Vec<Character>, Error> {
        let mut result = db
            .query("SELECT * FROM character WHERE account_id = $id") // TODO: query as graph
            .bind(("id", id))
            .await?;

        result.take(0)
    }

    pub async fn get_by_id(db: &Surreal<Any>, id: &String) -> Result<Option<Character>, Error> {
        let mut result = db
            .query(r#"
                SELECT 
                *, equipped_gear.*, inventory.*
                FROM type::thing("character", $id)
            "#) // TODO: query as graph
            .bind(("id", id))
            .await?;

        result.take(0)
    }   

    pub async fn create(
        db: &Surreal<Any>,
        name: &String,
        class_id: &String,
        account_id: &String,
    ) -> Result<String, &'static str> {
        let Ok(Some(mut class)) = db.select::<Option<Class>>(("class", class_id)).await else {
            return Err("Class not found");
        };

        class.id = None;

        let Ok(mut result) = db // TODO: query as graph
            .query("INSERT INTO character (name, class, experience, account_id, inventory, equipped_gear) VALUES ($name, $class, $experience, $account_id, $inventory, $equipped_gear) RETURN id;")
            .bind(("experience",    0                 ))
            .bind(("name",          name              ))
            .bind(("class",         class             ))
            .bind(("account_id",    account_id        ))
            .bind(("inventory",     Vec::<Gear>::new()))
            .bind(("equipped_gear", Vec::<Gear>::new()))
            .await else {
                return Err("Error creating character (E1001)");
            };

        let id = match result.take::<Option<HashMap<String, Thing>>>(0) {
            Ok(Some(id)) => id,
            Ok(None) => return Err("Error creating character (E1002)"),
            Err(e) => {
                dbg!(e);
                return Err("Error creating character (E1003)");
            }
        };

        let id = id.get("id").unwrap();

        Ok(id.id.to_raw())
    }

    pub async fn delete_by_id(db: &Surreal<Any>, id: &String) -> Result<(), Error> {
        db.query("DELETE type::thing(\"character\", $id)").bind(("id", id)).await?;

        Ok(())
    }
}
