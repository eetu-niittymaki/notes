use std::path::Path;

use libsql::{Builder, Connection};

use crate::db::create_tables::create_tables;

pub use notes::NotesRepository;
pub use tags::TagsRepository;
pub use search::SearchRepository;

pub mod notes;
pub mod tags;
pub mod search;
pub mod create_tables;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub async fn open(path: &Path) -> libsql::Result<Self> {
        let db = Builder::new_local(path)
            .build()
            .await?;

        let conn = db.connect()?;

        create_tables(&conn).await?;

        Ok(Self { conn })
    }

    pub fn notes(&self) -> NotesRepository<'_> {
        NotesRepository::new(&self.conn)
    }

    pub fn tags(&self) -> TagsRepository<'_> {
        TagsRepository::new(&self.conn)
    }

    pub fn search(&self) -> SearchRepository<'_> {
        SearchRepository::new(&self.conn)
    }
}