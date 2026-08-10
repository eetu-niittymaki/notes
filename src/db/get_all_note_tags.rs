use std::collections::HashMap;

use rusqlite::{Connection, Result};

use crate::models::Tag;

pub fn get_all_note_tags(
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