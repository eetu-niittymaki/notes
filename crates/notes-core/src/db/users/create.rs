use libsql::{Connection, params};

use crate::error::Result;
use crate::models::user::User;

pub async fn create(
    conn: &Connection,
    username: &str,
    password_hash: &str,
) -> Result<User> {
    let mut rows = conn
        .query(
            "INSERT INTO users (username, password_hash)
             VALUES (?1, ?2)
             RETURNING id, username, password_hash, created_at",
            params![username, password_hash],
        )
        .await?;

    let row = rows
        .next()
        .await?
        .expect("Failed to create user");

    Ok(User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
        created_at: row.get(3)?
    })
}