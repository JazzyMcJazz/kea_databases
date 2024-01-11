pub mod account_repo;
pub mod character_repo;
pub mod class_repo;
pub mod item_piece_repo;
pub mod enums;
pub mod models;

use async_trait::async_trait;
use surrealdb::{engine::any::Any, Surreal};

#[async_trait]
pub trait GraphRepo {
    async fn graphia_init(&self);
    async fn graphia_clear(&self, database: &str);
}

#[async_trait]
impl GraphRepo for Surreal<Any> {
    async fn graphia_clear(&self, database: &str) {
        self.query(format!("REMOVE DATABASE {};", database))
            .await
            .unwrap();
    }

    async fn graphia_init(&self) {
        let query = include_str!("./init.surql");
        let mut result = self.query(query).await.unwrap_or_else(|e| {
            panic!("Failed to initialize database: {}", e);
        });

        let errors = result.take_errors();
        if !errors.is_empty() {
            dbg!(errors);
        }
    }
}
