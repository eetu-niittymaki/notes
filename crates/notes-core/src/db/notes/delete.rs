use libsql::Connection;

use crate::error::Result;

use crate::models::note::NoteSelector;

pub async fn delete(
    conn: &Connection,
    selector: &NoteSelector<'_>
) -> Result<u64> {
    match selector {
        NoteSelector::Id(id) => {
            Ok(conn.execute(
                "DELETE FROM notes WHERE id = ?1",
            [*id],
            ).await?)
        }

        NoteSelector::Title(title) => {
            Ok(conn.execute(
                "DELETE FROM notes WHERE title = ?1",
            [*title],
            ).await?)
        }
    }
}

pub async fn delete_all(conn: &Connection) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM notes",())
        .await?)
}