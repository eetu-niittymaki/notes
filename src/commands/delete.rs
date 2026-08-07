use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    if cmd.all {
        conn.execute("DELETE FROM notes", [])?;  
        println!("All notes deleted!");
    } else if cmd.id.is_some()  {
        conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        [cmd.id],
        )?;
        println!("Note deleted!");
    } else {
        eprintln!("Please provide either an ID or -a/--all");
    }

    Ok(())
}