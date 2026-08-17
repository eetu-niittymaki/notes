use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::NoteSelector;

use crate::models::cli::GetCommand;

pub async fn get(cmd: GetCommand, db: &Database,) -> Result<()> {
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

    let note =  db.notes().get(selector).await?;
    let tags = db.tags().for_note(note.id).await?;

    println!("{} | {}", note.id, note.title);
    
    if !tags.is_empty() {
        for tag in tags {
            println!("  #{}", tag.name);
        }
    }

    println!("{}", note.content);

    Ok(())  
}