use rusqlite::{Connection, Result};

use crate::models::Tag;

pub fn get_all_tags(conn: &Connection) -> Result<Vec<Tag>> {
    let mut statement = conn.prepare(
        "SELECT id, name
         FROM tags
         ORDER BY name ASC"
    )?;

    let tags = statement
    .query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(tags)
}