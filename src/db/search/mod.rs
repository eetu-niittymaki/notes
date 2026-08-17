use libsql::Connection;

use crate::error::Result;
use crate::models::note::Note;

mod search;

pub struct SearchRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }  
    
    pub async fn notes(&self, title: Option<&str>, content: Option<&str>) -> Result<Vec<Note>> {
        search::notes(self.conn, title, content).await
    }

    pub async fn tags(&self, tag: &str) -> Result<Vec<Note>> {
        search::tags(self.conn, &tag).await
    }
}