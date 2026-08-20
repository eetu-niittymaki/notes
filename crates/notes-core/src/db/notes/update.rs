use libsql::Connection;

use crate::error::Result;

use crate::models::note::NoteUpdate;

pub async fn update(
    conn: &Connection,
    id: i64,
    update: NoteUpdate,
) -> Result<u64> {
    match (id, update) {
        (id, NoteUpdate::Title(new_title)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_title, id),
            ).await?)
        }

        (id, NoteUpdate::Content(new_content)) => {
            Ok(conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (new_content, id),
            ).await?)
        }
    }
}