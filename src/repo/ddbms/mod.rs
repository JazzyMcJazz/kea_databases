pub mod account_repo;
pub mod models;

use async_trait::async_trait;
use surrealdb::{engine::any::Any, Surreal};

#[async_trait]
pub trait Repo {
    async fn surreal_init(&self);
    async fn surreal_clear(&self);
}

#[async_trait]
impl Repo for Surreal<Any> {
    async fn surreal_clear(&self) {
        self.query(
            "
            REMOVE TABLE account;
        ",
        )
        .await
        .unwrap();
    }

    async fn surreal_init(&self) {
        self.query(
            "
            DEFINE INDEX accountUsernameIndex ON TABLE account COLUMNS username UNIQUE;
        ",
        )
        .await
        .unwrap();
    }
}
