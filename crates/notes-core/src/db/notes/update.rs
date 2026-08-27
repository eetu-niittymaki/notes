use libsql::Connection;

use crate::error::Result;

use crate::models::note::NoteUpdate;

pub async fn update(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    update: NoteUpdate,
) -> Result<u64> {
    match (note_id, update) {
        (note_id, NoteUpdate::Title(new_title)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_title, note_id, user_id),
            ).await?)
        }

        (note_id, NoteUpdate::Content(new_content)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_content, note_id, user_id),
            ).await?)
        }
    }
}