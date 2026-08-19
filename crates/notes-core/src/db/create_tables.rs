use libsql::Connection;

pub async fn create_tables(conn: &Connection) -> libsql::Result<()> {
    conn.execute_transactional_batch(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            favorite BOOLEAN NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS note_tags (
            note_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,

            PRIMARY KEY (note_id, tag_id),

            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
        "#,
    )
    .await?;

    Ok(())
}