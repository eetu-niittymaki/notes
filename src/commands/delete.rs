
use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;
use crate::db::delete_all_notes::delete_all_notes;
use crate::db::delete_note::delete_note;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    if cmd.all {
        let rows = delete_all_notes(conn)?;
        if rows > 0 {
            println!("All notes deleted!");
        } else {
            println!("No notes to delete")
        }

        return Ok(())
    } 

    if cmd.title.is_some() && cmd.id.is_some() {
        eprintln!("Please provide either an ID or a title, not both.");
        return Ok(());
    }
    
    if cmd.title.is_some() || cmd.id.is_some()  {
        let rows = delete_note(conn, cmd.title.as_deref(), cmd.id)?;
        if rows == 1 {
            println!("Note deleted!") 
        } else  {
            println!("No matching note found")
        }
        
    } else {
        eprintln!("Please provide either an ID or -a/--all");
    }

    Ok(())
}