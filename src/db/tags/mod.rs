use std::collections::HashMap;

use libsql::Connection;

use crate::error::Result;

use crate::models::{Tag, TagWithCount};

mod get;
mod create;
mod delete;

pub struct TagsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TagsRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }  

    pub async fn add(&self, id: i64, tag: &str) -> Result<u64> {
        create::add(self.conn, id, tag).await
    }
    
    pub async fn all(&self) -> Result<Vec<TagWithCount>> {
        get::all(self.conn).await
    }

    pub async fn all_for_notes(&self) -> Result<HashMap<i64, Vec<Tag>>> {
        get::all_for_notes(self.conn).await
    }

    pub async fn for_note(&self, note_id: i64) -> Result<Vec<Tag>> {
        get::for_note(self.conn, note_id).await
    }

    pub async fn delete(&self, name: &str) -> Result<u64> {
        delete::one(self.conn, name).await
    }
}