use libsql::Connection;

use crate::error::Result;
use crate::error::Error::UserNotFound;
use crate::models::user::User;

pub async fn by_id(
    conn: &Connection,
    user_id: i64,
) -> Result<Option<User>> {
    let mut rows = conn
        .query("
            SELECT *
            FROM users
            WHERE id = ?1",
            [user_id],
        )
        .await?;

    let row = match rows.next().await? {
        Some(row) => row,
        None => {
            return Err(UserNotFound.into());
        }
    };

    Ok(Some(User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
        created_at: row.get(3)?,
    }))
}

pub async fn by_username(
    conn: &Connection,
    username: &str,
) -> Result<Option<User>> {
    let mut rows = conn
        .query("
            SELECT *
            FROM users
            WHERE username = ?1",
            [username],
        )
        .await?;

    let row = match rows.next().await? {
        Some(row) => row,
        None => {
            return Err(UserNotFound.into());
        }
    };

    Ok(Some(User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
        created_at: row.get(3)?,
    }))
}