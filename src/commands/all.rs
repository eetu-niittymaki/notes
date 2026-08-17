use crate::error::Result;

use crate::cli::AllCommand;

use crate::db::Database;

pub async fn all(cmd: AllCommand, db: &Database,) -> Result<()> {
    let notes = db.notes().get_all().await?;

    if notes.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    let note_tags = db.tags().all_for_notes().await?;

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