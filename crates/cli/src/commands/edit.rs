use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::{NoteSelector, NoteUpdate};

use crate::models::cli::{EditCommand, EditField};

use crate::utils::text_editor::text_editor;

pub async fn edit(cmd: EditCommand, db: &Database,) -> Result<()> {
    let selector = NoteSelector::Id(cmd.id);

    let note = db.notes().get(selector.clone()).await?;

    let update = match cmd.field {
        EditField::Title => {
            let new_title = text_editor(
                note.title,
                 Some("Edit Title".to_string())
            );
            NoteUpdate::Title(new_title)
        }

        EditField::Content => {
            let new_content = text_editor(
                note.content,
                 Some("Edit Contetent".to_string())
                );
            NoteUpdate::Content(new_content)
        }
    };

    let updated = db.notes().update(selector, update).await?;

    if updated == 1 {
        println!("Note updated successfully!");
    }

    Ok(())
}