use rusqlite::Result;

use crate::cli::AllCommand;

use crate::db::Database;

pub fn all(cmd: AllCommand, db: &Database,) -> Result<()> {
    let notes = db.notes().get_all()?;

    if notes.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    let note_tags = db.tags().all_for_notes()?;

    println!("All Notes:");
    println!("----------");

    for note in notes {
        println!("{} | {}", note.id, note.title);

        if let Some(tags) = note_tags.get(&note.id) {
            for tag in tags {
                println!("  #{}", tag.name);
            }
        }

        if cmd.content {
            println!("\n{}", note.content);
        }
    }

    Ok(())
}