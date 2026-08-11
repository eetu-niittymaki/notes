use rusqlite::{Connection, Result};

use crate::cli::{EditCommand, EditField};
use crate::db::update_note::update_note;
use crate::db::get_note::get_note;
use crate::utils::editor::editor;

use crate::models::{NoteSelector, NoteUpdate};

pub fn edit(cmd: EditCommand, conn: &Connection) -> Result<()> {
    let selector = NoteSelector::Id(cmd.id);

    let note = get_note(conn, &selector)?;

    let update = match cmd.field {
        EditField::Title => {
            let new_title = editor(note.title);
            NoteUpdate::Title(new_title)
        }

        EditField::Content => {
            let new_content = editor(note.content);
            NoteUpdate::Content(new_content)
        }
    };

    let updated = update_note(conn, &selector, update)?;

    if updated == 1 {
        println!("Note updated successfully!");
    }

    Ok(())
}