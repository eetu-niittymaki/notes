use rusqlite::{Connection, Result};

use crate::models::NoteSelector;
use crate::models::NoteUpdate;

pub fn update_note(
    conn: &Connection,
    selector: NoteSelector,
    update: NoteUpdate,
) -> Result<usize> {
    match (selector, update) {
        (NoteSelector::Id(id), NoteUpdate::Title(new_title)) => {
            conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_title, id),
            )
        }

        (NoteSelector::Id(id), NoteUpdate::Content(new_content)) => {
            conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_content, id),
            )
        }

        (NoteSelector::Title(title), NoteUpdate::Title(new_title)) => {
            conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE title = ?2",
                (new_title, title),
            )
        }

        (NoteSelector::Title(title), NoteUpdate::Content(new_content)) => {
            conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE title = ?2",
                (new_content, title),
            )
        }
    }
}