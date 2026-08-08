use std::io::{self, Write};

use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;
use crate::db::delete_all_notes::delete_all_notes;
use crate::db::delete_note::delete_note;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    if cmd.all {
        let mut confirm = String::new();
        print!("Delete all notes? y/n: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read input");

        confirm = confirm.trim().to_lowercase();
    
        if confirm == "y" || confirm == "yes" {
            let rows = delete_all_notes(conn)?;
            if rows > 0 {
                println!("All notes deleted!");
            } else {
                println!("No notes to delete")
            }

            return Ok(())
        } else {
            println!("Note deletion stopped");
            std::process::exit(0);
        }
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