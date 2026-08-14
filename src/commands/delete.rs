use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;
use crate::db::delete_all_notes::delete_all_notes;
use crate::db::delete_note::delete_note;
use crate::utils::get_user_input::get_user_input;

use crate::models::NoteSelector;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    if cmd.all {
        println!("Delete all notes? y/n");
        let confirm = get_user_input().trim().to_lowercase();

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

    let selector = match (cmd.id, cmd.title.as_deref()) {
        (Some(id), None) => NoteSelector::Id(id),

        (None, Some(title)) => NoteSelector::Title(title),

        (None, None) => {
            eprintln!("Please provide either --id or --title");
            return Ok(());
        }

        (Some(_), Some(_)) => {
            eprintln!("Please provide either --id or --title, not both");
            return Ok(());
        }
    };
    
    let rows = delete_note(conn, selector)?;
    if rows == 1 {
        println!("Note deleted!") 
    } else  {
        println!("No matching note found")
    }
        


    Ok(())
}