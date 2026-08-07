use rusqlite::{Connection, Result};

pub fn all(conn: &Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT id, title, content FROM notes")?;

    let rows: Vec<_> = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?
    .collect::<Result<_,_>>()?;

    if !rows.is_empty() {
        println!("All notes");
        println!("---------");

        for row in rows {
            let (_id, title, content) = row;
            println!("{}: {}", title, content);
        }
    } else {
        println!("No notes found")
    }

    Ok(())
}