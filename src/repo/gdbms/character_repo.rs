use std::collections::HashMap;

use surrealdb::{engine::any::Any, sql::Thing, Error, Surreal};

use super::models::{Character, Class, Gear};

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn get_by_account_id(db: &Surreal<Any>, id: String) -> Result<Vec<Character>, Error> {
        let mut result = db
            .query(r#"
                SELECT 
                    *,
                    array::first(->is_a.out.*) as class,
                    string::split(type::string(array::first(<-has<-account.id)), ":")[1] as account_id,
                    ->equipped->gear->is_instance_of->item.* as equipped_gear,
                    ->unequipped->gear->is_instance_of->item.* as inventory
                FROM type::thing("account", $id)->has->character;
            "#)
            .bind(("id", id))
            .await?;

        result.take(0)
    }

    pub async fn get_by_id(db: &Surreal<Any>, id: &String) -> Result<Option<Character>, Error> {
        let mut result = db
            .query(r#"
                SELECT 
                    *,
                    string::split(type::string(array::first(<-has<-account.id)), ":")[1] as account_id,
                    array::first(->is_a.out.*) as class,
                    ->equipped->gear->is_instance_of->item.* as equipped_gear,
                    ->unequipped->gear->is_instance_of->item.* as inventory
                FROM type::thing("character", $id);
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

        let query = include_str!("queries/create_character.surql");
        let Ok(mut result) = db
            .query(query)
            .bind(("account_id", account_id))
            .bind(("class_id",   class.id     ))
            .bind(("name",       name      ))
            .bind(("experience", 0         ))
            .await else {
                return Err("Error creating character (E1001)");
            };

        let errors = result.take_errors();
        if errors.len() > 0 {
            dbg!(errors);
            return Err("Error creating character (E1002)");
        }

        let id = match result.take::<Option<Thing>>(0) {
            Ok(Some(id)) => id,
            Ok(None) => return Err("Error creating character (E1003)"),
            Err(e) => {
                dbg!(e);
                return Err("Error creating character (E1004)");
            }
        };

        Ok(id.id.to_raw())
    }

    pub async fn delete_by_id(db: &Surreal<Any>, id: &String) -> Result<(), Error> {
        db.query("DELETE type::thing(\"character\", $id)").bind(("id", id)).await?;

        Ok(())
    }
}
