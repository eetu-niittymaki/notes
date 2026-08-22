use libsql::Connection;

pub async fn create_tables(conn: &Connection) -> libsql::Result<()> {
    conn.execute_transactional_batch(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        -- Only for testing
        INSERT INTO users (username, password_hash) 
        VALUES('Testi', '12345')
        ON CONFLICT (username) DO NOTHING;

        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL UNIQUE,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            favorite BOOLEAN NOT NULL DEFAULT 0,

            FOREIGN KEY (user_id) REFERENCES users(id)
        );

        CREATE INDEX IF NOT EXISTS idx_notes_user_id ON notes(user_id);

        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,

            FOREIGN KEY (user_id) REFERENCES users(id),
            UNIQUE (user_id, name)
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