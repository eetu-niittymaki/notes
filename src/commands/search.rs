use rusqlite::{Connection, Result};

use crate::cli::SearchCommand;

pub fn search(cmd: SearchCommand, conn: &Connection) -> Result <()> {
    let pattern = format!("%{}%", cmd.content);

    let mut statement = conn.prepare(
    "SELECT title, content FROM notes
        WHERE content LIKE ?1",
    )?;

    let rows = statement.query_map([pattern], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    println!("Found notes");
    println!("-----------");

    for row in rows {
        let (id, note) = row?;
        println!("{}: {}", id, note);
    }

    Ok(())
}