pub mod notes;
pub mod tags;
pub mod search;
pub mod create_tables;

use rusqlite::Connection;

pub use notes::NotesRepository;
pub use tags::TagsRepository;
pub use search::SearchRepository;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
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