use std::collections::HashMap;

use surrealdb::{engine::any::Any, Error, Surreal};

use crate::repo::ddbms::models::Account;

pub struct SurrealAccountRepo;

impl SurrealAccountRepo {
    pub async fn find_by_credentials(
        db: &Surreal<Any>,
        username: String,
        password: String,
    ) -> Result<Option<Account>, Error> {
        let mut result = db
            .query(
                "
                SELECT * FROM account WHERE 
                username = $username AND 
                crypto::argon2::compare(password, $password)
            ",
            )
            .bind(("username", username))
            .bind(("password", password))
            .await?;

        let account: Option<Account> = result.take(0)?;
        Ok(account)
    }

    pub async fn create_account(
        db: &Surreal<Any>,
        username: String,
        password: String,
    ) -> Result<Account, &'static str> {
        let Ok(mut result) = db
            .query("SELECT username FROM account WHERE username = $username")
            .bind(("username", &username))
            .await
        else {
            return Err("Failed to create account (E1001)");
        };

        let existing: Option<HashMap<String, String>> = match result.take(0) {
            Ok(existing) => existing,
            Err(_) => return Err("Failed to create account (E1002)"),
        };

        if existing.is_some() {
            return Err("Username already exists");
        }

        let Ok(mut result) = db
            .query(
                r#"
                CREATE account CONTENT {
                    username: $username,
                    password: crypto::argon2::generate($password),
                    last_login: time::now(),
                }
            "#,
            )
            .bind(("password", &password))
            .bind(("username", &username))
            .await
        else {
            return Err("Failed to create account (E1003)");
        };

        let created: Option<Account> = match result.take(0) {
            Ok(created) => created,
            Err(_) => return Err("Failed to create account (E1004)"),
        };

        Ok(created.unwrap())
    }
}
