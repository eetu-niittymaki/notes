use std::collections::HashMap;

use rusqlite::{Connection, Result};

use crate::models::{Tag, TagWithCount};

pub fn all(conn: &Connection) -> Result<Vec<TagWithCount>> {
    let mut statement = conn.prepare(
    "SELECT
        tag.id,
        tag.name,
        COUNT(note_tag.note_id)
    FROM tags AS tag
    LEFT JOIN note_tags AS note_tag
        ON tag.id = note_tag.tag_id
    GROUP BY tag.id, tag.name
    ORDER BY tag.name ASC",
    )?;

    let tags = statement
        .query_map([], |row| {
            Ok(TagWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                note_count: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tags)
}

pub fn all_for_notes(
    conn: &Connection,
) -> Result<HashMap<i64, Vec<Tag>>> {
    let mut statement = conn.prepare(
        "SELECT
            note_tag.note_id,
            tag.id,
            tag.name
         FROM note_tags AS note_tag
         JOIN tags AS tag
            ON tag.id = note_tag.tag_id
         ORDER BY note_tag.note_id, tag.name",
    )?;

    let mut note_tags: HashMap<i64, Vec<Tag>> = HashMap::new();

    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            Tag {
                id: row.get(1)?,
                name: row.get(2)?,
            },
        ))
    })?;

    for row in rows {
        let (note_id, tag) = row?;
        note_tags.entry(note_id).or_default().push(tag);
    }

    Ok(note_tags)
}

pub fn for_note(
    conn: &Connection, 
    note_id: i64 
) -> Result<Vec<Tag>> {
    let mut statement = conn.prepare(
        "SELECT tag.id, tag.name
         FROM tags AS tag
         JOIN note_tags AS note_tag
             ON tag.id = note_tag.tag_id
         WHERE note_tag.note_id = ?1
         ORDER BY tag.name"
    )?;

    let tags = statement
        .query_map([note_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tags)
}

