use libsql::Connection;

use crate::error::Result;

use crate::models::note::NoteUpdate;

pub async fn update(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    update: NoteUpdate,
) -> Result<u64> {
    let rows_matched = match update {
        NoteUpdate::Title(new_title) => {
            conn.execute(
                "UPDATE notes
                 SET title = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_title, note_id, user_id),
            ).await?
        }

        NoteUpdate::Content(new_content) => {
            conn.execute(
                "UPDATE notes
                 SET content = ?1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2
                 AND user_id = ?3",
                (new_content, note_id, user_id),
            ).await?
        }
    };

    Ok(rows_matched)
}