use serde::{Deserialize, Serialize};

pub trait Claims {
    fn new(sub: String, username: String) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdbClaims {
    pub sub: i32,
    username: String,
    exp: usize,
}

impl Claims for RdbClaims {
    fn new(sub: String, username: String) -> Self {
        let sub = sub.parse::<i32>().unwrap();
        let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
        Self {
            sub,
            username,
            exp: exp as usize,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DdbClaims {
    pub sub: String,
    username: String,
    exp: usize,
}

impl Claims for DdbClaims {
    fn new(sub: String, username: String) -> Self {
        let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
        Self {
            sub,
            username,
            exp: exp as usize,
        }
    }
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
