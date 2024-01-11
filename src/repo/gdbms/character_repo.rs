use surrealdb::{engine::any::Any, sql::Thing, Error, Surreal};

use super::models::{Character, Class};

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn exists_by_id_and_account_id(
        db: &Surreal<Any>,
        character_id: &String,
        account_id: &String,
    ) -> Result<bool, Error> {
        let mut result = db
            .query(r#"
                LET $acc = type::thing("account", $account_id);
                LET $char = type::thing("character", $character_id);
                LET $result = SELECT out.id AS id FROM has WHERE in.id IS $acc AND out.id IS $char;
                array::len($result) IS 1;
            "#)
            .bind(("account_id", account_id))
            .bind(("character_id", character_id))
            .await?;

        match result.take::<Option<bool>>(3) {
            Ok(account_id) => Ok(account_id.is_some()),
            Err(e) => {
                dbg!(&e);
                Err(e)
            }
        }
    }

    pub async fn get_by_account_id(db: &Surreal<Any>, id: String) -> Result<Vec<Character>, Error> {
        let mut result = db
            .query(r#"
                SELECT 
                    *,
                    array::first(->is_a.out.*) as class,
                    string::split(type::string(array::first(<-has<-account.id)), ":")[1] as account_id,
                    [] as equipped_gear,
                    [] as inventory
                FROM type::thing("account", $id)->has->character;
            "#)
            .bind(("id", id))
            .await?;

        result.take(0)
    }

    pub async fn get_by_id(db: &Surreal<Any>, id: &String) -> Result<Option<Character>, Error> {
        let query = include_str!("queries/select_character_details.surql");
        
        let mut result = db
            .query(query) 
            .bind(("id", id))
            .await?;

        result.take(1)
    }   

    pub async fn create(
        db: &Surreal<Any>,
        name: &String,
        class_id: &String,
        account_id: &String,
    ) -> Result<String, &'static str> {
        let Ok(Some(class)) = db.select::<Option<Class>>(("class", class_id)).await else {
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
