use libsql::Connection;

use crate::error::Result;

use crate::models::NoteSelector;
use crate::models::NoteUpdate;

pub async fn update(
    conn: &Connection,
    selector: &NoteSelector<'_>,
    update: NoteUpdate,
) -> Result<u64> {
    match (selector, update) {
        (NoteSelector::Id(id), NoteUpdate::Title(new_title)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_title, *id),
            ).await?)
        }

        (NoteSelector::Id(id), NoteUpdate::Content(new_content)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_content, *id),
            ).await?)
        }

        (NoteSelector::Title(title), NoteUpdate::Title(new_title)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE title = ?2",
                (new_title, *title),
            ).await?)
        }

        (NoteSelector::Title(title), NoteUpdate::Content(new_content)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE title = ?2",
                (new_content, *title),
            ).await?)
        }
    }
}