use rusqlite::{Connection, Result};

pub fn all(conn: &Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT id, note FROM notes")?;

    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    for row in rows {
        let (id, note) = row?;
        println!("{}: {}", id, note);
    }

    Ok(())
}