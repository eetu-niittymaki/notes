use rusqlite::Result;

use crate::cli::GetCommand;

use crate::db::Database;

use crate::models::NoteSelector;

pub fn get(cmd: GetCommand, db: &Database,) -> Result<()> {
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

    let note =  db.notes().get(selector)?;
    let tags = db.tags().for_note(note.id)?;

    println!("{} | {}", note.id, note.title);
    
    if !tags.is_empty() {
        for tag in tags {
            println!("  #{}", tag.name);
        }
    }

    println!("{}", note.content);

    Ok(())  
}