use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    id: Thing,
    tb: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Thing,
    pub username: String,
    pub password: String,
}
