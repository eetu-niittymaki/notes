use libsql::{Connection, params};

use crate::error::{Result, Error};

pub async fn create(
    conn: &Connection,
    user_id: i64,
    title: &str,
    content: &str,
) -> Result<i64> {
    let mut rows = conn
        .query(
            "INSERT INTO notes (user_id, title, content)
             VALUES (?1, ?2, ?3)
             RETURNING id",
            params![user_id, title, content],
        )
        .await?;

    let row = rows
        .next()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(row.get(0)?)
}