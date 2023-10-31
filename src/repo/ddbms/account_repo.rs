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
    ) -> Result<Account, Error> {
        let mut result = db
            .query(
                "
            let $password_hash = crypto::argon2::generate($password);

            CREATE account CONTENT {
                username: $username,
                password: $password_hash,
                last_login: time::now(),
            }
        ",
            )
            .bind(("password", &password))
            .bind(("username", &username))
            .await?;

        let created: Option<Account> = match result.take(1) {
            Ok(created) => created,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(created.unwrap())
    }
}
