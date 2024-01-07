use surrealdb::{engine::any::Any, Error, Surreal};

use super::models::Class;

pub struct ClassRepo;

impl ClassRepo {
    pub async fn all(db: &Surreal<Any>) -> Result<Vec<Class>, Error> {
        db.select("class").await
    }
}
