use rusqlite::{Connection, Result};

use crate::cli::GetCommand;
use crate::db::get_note::get_note;
use crate::db::get_tags_for_note::get_tags_for_note;

use crate::models::NoteSelector;

pub fn get(cmd: GetCommand, conn: &Connection) -> Result<()> {
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

    let note =  get_note(conn, selector)?;
    let tags = get_tags_for_note(conn, note.id)?;

    println!("{} | {}", note.id, note.title);
    
    if !tags.is_empty() {
        for tag in tags {
            println!("  #{}", tag.name);
        }
    }

    println!("{}", note.content);

    Ok(())  
}