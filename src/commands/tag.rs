use crate::error::Result;

use crate::cli::TagCommand;

use crate::db::Database;

pub fn tag(cmd: TagCommand, db: &Database,) -> Result<()> {
    match cmd {
        TagCommand::Add { note_id, name } => {
            let rows = db.tags().add(note_id, &name)?;

            if rows == 1 {
                println!("Tag '{}' added to note {}!", name, note_id);
            } else {
                println!("Note already has that tag");
            }
        }

        TagCommand::Delete { name } => {
            let rows = db.tags().delete(&name)?;

            if rows == 1 {
                println!("Tag '{}' deleted!", name);
            } else {
                println!("No matching tag found");
            }
        }

        TagCommand::List => {
            let tags = db.tags().all()?;

            if tags.is_empty() {
                println!("No tags found");
                return Ok(());
            }

            println!("All Tags:");
            println!("---------");

            for tag in tags {
                let word = if tag.note_count > 1 { "notes" } else { "note" };
                println!("{} ({} {})", tag.name, tag.note_count, word);
            }
        }
    }

    Ok(())
}
