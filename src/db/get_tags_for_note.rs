use rusqlite::{Connection, Result};

use crate::models::Tag;

pub fn get_tags_for_note(
    conn: &Connection, 
    note_id: i64 
) -> Result<Vec<Tag>>{
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
                name: row.get(1)?,
                note_count: 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tags)
}