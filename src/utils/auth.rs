use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RdbClaims {
    pub sub: i32,
    username: String,
    exp: usize,
}

impl RdbClaims {
    pub fn new(sub: i32, username: String) -> Self {
        let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
        Self { sub, username, exp: exp as usize }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DdbClaims {
    pub sub: i32,
    username: String,
    exp: usize,
}

impl DdbClaims {
    // pub fn new(sub: i32, username: String) -> Self {
    //     let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
    //     Self { sub, username, exp: exp as usize }
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdbClaims {
    sub: i32,
    username: String,
    exp: usize,
}

impl GdbClaims {
    // pub fn new(sub: i32, username: String) -> Self {
    //     let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
    //     Self { sub, username, exp: exp as usize }
    // }
}