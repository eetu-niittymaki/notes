use std::path::Path;

use rusqlite::Connection;

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
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        create_tables(&conn)?;

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