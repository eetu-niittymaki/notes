use std::collections::HashMap;

use libsql::Connection;

use crate::error::Result;

use crate::models::tag::{
    Tag, 
    TagWithCount,
    CreateTagQuery,
    DeleteTagQuery,
};

pub mod get;
mod create;
mod delete;

pub struct TagsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TagsRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }  

    pub async fn add(&self, tag: CreateTagQuery) -> Result<u64> {
        create::add(self.conn, 1, tag.note_id, &tag.name,).await
    }
    
    pub async fn all(&self) -> Result<Vec<TagWithCount>> {
        get::all(self.conn, 1).await
    }

    pub async fn all_for_notes(&self) -> Result<HashMap<i64, Vec<Tag>>> {
        get::all_for_notes(self.conn, 1).await
    }

    pub async fn for_note(&self, note_id: i64) -> Result<Vec<Tag>> {
        get::for_note(self.conn, note_id, 1).await
    }

    pub async fn delete(&self, tag: DeleteTagQuery) -> Result<u64> {
        delete::delete(self.conn, &tag.name).await
    }
}