use std::collections::HashMap;

use libsql::Connection;

use crate::error::Result;
use crate::models::tag::{Tag, TagWithCount};

pub async fn all(conn: &Connection) -> Result<Vec<TagWithCount>> {
    let statement = conn
        .prepare(
            r#"
            SELECT
                tag.id,
                tag.name,
                COUNT(note_tag.note_id)
            FROM tags AS tag
            LEFT JOIN note_tags AS note_tag
                ON tag.id = note_tag.tag_id
            GROUP BY tag.id, tag.name
            ORDER BY tag.name ASC
            "#,
        )
        .await?;

    let mut rows = statement.query(()).await?;

    let mut tags = Vec::new();

    while let Some(row) = rows.next().await? {
        tags.push(TagWithCount {
            id: row.get(0)?,
            name: row.get(1)?,
            note_count: row.get(2)?,
        });
    }

    Ok(tags)
}

pub async fn all_for_notes(
    conn: &Connection,
) -> Result<HashMap<i64, Vec<Tag>>> {
    let statement = conn
        .prepare(
            r#"
            SELECT
                note_tag.note_id,
                tag.id,
                tag.name
            FROM note_tags AS note_tag
            JOIN tags AS tag
                ON tag.id = note_tag.tag_id
            ORDER BY note_tag.note_id, tag.name
            "#,
        )
        .await?;

    let mut rows = statement.query(()).await?;

    let mut note_tags: HashMap<i64, Vec<Tag>> = HashMap::new();

    while let Some(row) = rows.next().await? {
        let note_id: i64 = row.get(0)?;

        let tag = Tag {
            id: row.get(1)?,
            name: row.get(2)?,
        };

        note_tags
            .entry(note_id)
            .or_default()
            .push(tag);
    }

    Ok(note_tags)
}

pub async fn for_note(
    conn: &Connection,
    note_id: i64,
) -> Result<Vec<Tag>> {
    let statement = conn
        .prepare(
            r#"
            SELECT
                tag.id,
                tag.name
            FROM tags AS tag
            JOIN note_tags AS note_tag
                ON tag.id = note_tag.tag_id
            WHERE note_tag.note_id = ?1
            ORDER BY tag.name
            "#,
        )
        .await?;

    let mut rows = statement
        .query([note_id])
        .await?;

    let mut tags = Vec::new();

    while let Some(row) = rows.next().await? {
        tags.push(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }

    Ok(tags)
}