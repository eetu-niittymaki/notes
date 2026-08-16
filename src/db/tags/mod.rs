use std::collections::HashMap;

use rusqlite::Connection;

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

    pub fn add(&self, id: i64, tag: &str) -> Result<usize> {
        Ok(create::add(self.conn, id, tag)?)
    }
    
    pub fn all(&self) -> Result<Vec<TagWithCount>> {
        Ok(get::all(self.conn)?)
    }

    pub fn all_for_notes(&self) -> Result<HashMap<i64, Vec<Tag>>> {
        Ok(get::all_for_notes(self.conn)?)
    }

    pub fn for_note(&self, note_id: i64) -> Result<Vec<Tag>> {
        Ok(get::for_note(self.conn, note_id)?)
    }

    pub fn delete(&self, name: &str) -> Result<usize> {
        Ok(delete::one(self.conn, name)?)
    }
}