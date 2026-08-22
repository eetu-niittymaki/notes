use libsql::Connection;

use crate::error::Result;

use crate::models::note::NoteUpdate;

pub async fn update(
    conn: &Connection,
    id: i64,
    user_id: i64,
    update: NoteUpdate,
) -> Result<u64> {
    match (id, update) {
        (id, NoteUpdate::Title(new_title)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_title, id, user_id),
            ).await?)
        }

        (id, NoteUpdate::Content(new_content)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_content, id, user_id),
            ).await?)
        }
    }
}