use rusqlite::{Connection, Result};

use crate::cli::UpdateCommand;
use crate::db::update_note::update_note;

use crate::models::NoteSelector;
use crate::models::NoteUpdate;

pub fn update(cmd: UpdateCommand, conn: &Connection) -> Result<()> {
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

    let update = match (
        cmd.new_title.as_deref(),
        cmd.new_content.as_deref(),
    ) {
        (Some(title), None) => NoteUpdate::Title(title),

        (None, Some(content)) => NoteUpdate::Content(content),

        (None, None) => {
            eprintln!("Please provide --new-title or --content");
            return Ok(());
        }

        (Some(_), Some(_)) => {
            eprintln!("Please provide either --new-title or --content, not both");
            return Ok(());
        }
    };

    let updated = update_note(conn, selector, update)?;

    if updated == 1 {
        println!("Note updated successfully!");
    } else {
        println!("No matching note found.");
    }

    Ok(())
}