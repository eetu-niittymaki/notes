use libsql::Connection;

use crate::error::{Result, Error};
use crate::models::history::{
    NoteHistory
};

fn parse_row(row: &libsql::Row) -> Result<NoteHistory> {
    Ok(NoteHistory {
        id: row.get(0)?,
        note_id: row.get(1)?,
        version_number: row.get(2)?,
        operation: row.get(3)?,
        title: row.get(4)?,
        content: row.get(5)?,
        created_at: row.get(6)?,
    })
}

pub async fn all(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
) -> Result<Vec<NoteHistory>> {
    let mut rows = conn
        .query(
            "SELECT id,
                    note_id,
                    version_number,
                    operation,
                    title,
                    content,
                    created_at
             FROM notes_history
             WHERE user_id = ?1
               AND note_id = ?2
             ORDER BY version_number DESC;",
            [user_id, note_id],
        )
        .await?;

    let mut history = Vec::new();

    while let Some(row) = rows.next().await? {
        history.push(parse_row(&row)?);
    }

    Ok(history)
}

pub async fn one(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    version_number: i64,
) -> Result<NoteHistory> {
    let mut rows = conn
        .query(
            "SELECT id,
                    note_id,
                    version_number,
                    operation,
                    title,
                    content,
                    created_at
             FROM notes_history
             WHERE user_id = ?1
               AND note_id = ?2
               AND version_number = ?3;",
            [user_id, note_id, version_number],
        )
        .await?;

    match rows.next().await? {
        Some(row) => parse_row(&row),
        None => Err(Error::NotFound.into()),
    }
}

pub async fn newest(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
) -> Result<NoteHistory> {
    let mut rows = conn
        .query(
            "SELECT id,
                    note_id,
                    version_number,
                    operation,
                    title,
                    content,
                    created_at
             FROM notes_history
             WHERE user_id = ?1
               AND note_id = ?2
             ORDER BY version_number DESC
             LIMIT 1;",
            [user_id, note_id],
        )
        .await?;

    match rows.next().await? {
        Some(row) => parse_row(&row),
        None => Err(Error::NotFound.into()),
    }
}