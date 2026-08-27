use libsql::Connection;

use crate::error::Result;
use crate::models::note::{Note, NoteSearchQuery};
use crate::models::tag::TagSearchQuery;

mod search;

pub struct SearchRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }  
    
    pub async fn notes(&self, user_id: i64, query: NoteSearchQuery) -> Result<Vec<Note>> {
        search::notes(self.conn, user_id, query.title.as_deref(), query.content.as_deref()).await
    }

    pub async fn tags(&self, user_id: i64, query: TagSearchQuery) -> Result<Vec<Note>> {
        search::tags(self.conn, user_id, &query.tag, ).await
    }
}