mod get;
mod create;
mod update;
mod delete;

use libsql::Connection;

use crate::error::Result;

use crate::models::{Note, NoteSelector, NoteUpdate};

pub struct NotesRepository<'a> {
    conn: &'a Connection,
}

impl<'a> NotesRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn create(&self, title: String, content: &str) -> Result<u64> {
        create::create(self.conn, &title, content).await
    }
    
    pub async fn get(&self, selector: NoteSelector<'_>) -> Result<Note> {
        get::one(self.conn, &selector).await
    }

    pub async fn get_all(&self) -> Result<Vec<Note>> {
        get::all(self.conn).await
    }

    pub async fn update(&self, selector: NoteSelector<'_>, updater: NoteUpdate) -> Result<u64> {
        update::update(self.conn, &selector, updater).await
    }

    pub async fn delete(&self, selector: NoteSelector<'_>) -> Result<u64> {
        delete::one(self.conn, &selector).await
    }

    pub async fn delete_all(&self) -> Result<u64> {
        delete::all(self.conn).await
    }
}