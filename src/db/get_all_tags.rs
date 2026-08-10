use rusqlite::{Connection, Result};

use crate::models::TagWithCount;

pub fn get_all_tags(conn: &Connection) -> Result<Vec<TagWithCount>> {
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
