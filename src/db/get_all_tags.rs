use rusqlite::{Connection, Result};

use crate::models::Tag;

pub fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>> {
    let mut statement = conn.prepare(
        "SELECT
            tag.id,
            tag.name,
            COUNT(note_tag.note_id)
         FROM tags AS tag
         LEFT JOIN note_tags AS note_tag
             ON tag.id = note_tag.tag_id
         GROUP BY tag.id
         ORDER BY tag.name ASC",
    )?;

    let tags = statement
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                note_count: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tags)
}
