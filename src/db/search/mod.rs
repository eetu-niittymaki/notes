use rusqlite::{Connection, Result};

use crate::models::Note;

mod search;

pub struct SearchRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }  
    
    pub fn notes(&self, title: Option<&str>, content: Option<&str>) -> Result<Vec<Note>> {
        search::notes(&self.conn, title, content)
    }

    pub fn tags(&self, tag: String) -> Result<Vec<Note>> {
        search::tags(&self.conn, tag)
    }
}