mod get;
mod create;
mod update;
mod delete;

use rusqlite::Connection;

use crate::error::Result;

use crate::models::{Note, NoteSelector, NoteUpdate};

pub struct NotesRepository<'a> {
    conn: &'a Connection,
}

impl<'a> NotesRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub fn create(&self, title: String, content: &str) -> Result<usize> {
        Ok(create::create(self.conn, &title, content)?)
    }
    
    pub fn get(&self, selector: NoteSelector) -> Result<Note> {
        Ok(get::one(self.conn, &selector)?)
    }

    pub fn get_all(&self) -> Result<Vec<Note>> {
        Ok(get::all(self.conn)?)
    }

    pub fn update(&self, selector: NoteSelector, updater: NoteUpdate) -> Result<usize> {
        Ok(update::update(self.conn, &selector, updater)?)
    }

    pub fn delete(&self, selector: NoteSelector) -> Result<usize> {
        Ok(delete::one(self.conn, &selector)?)
    }

    pub fn delete_all(&self) -> Result<usize> {
        Ok(delete::all(self.conn)?)
    }
}