use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i64,
    username: String,
    password_hash: String,
    created_at: String
    }

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser<'a> {
    username: &'a str,
    password_hash: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    username: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    id: i64,
}