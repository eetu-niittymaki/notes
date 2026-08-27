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

    pub async fn add(&self, user_id: i64, tag: CreateTagQuery) -> Result<u64> {
        create::add(self.conn, user_id, tag.note_id, &tag.name,).await
    }
    
    pub async fn all(&self, user_id: i64) -> Result<Vec<TagWithCount>> {
        get::all(self.conn, user_id).await
    }

    pub async fn all_for_notes(&self, user_id: i64) -> Result<HashMap<i64, Vec<Tag>>> {
        get::all_for_notes(self.conn, user_id).await
    }

    pub async fn for_note(&self, user_id: i64, note_id: i64) -> Result<Vec<Tag>> {
        get::for_note(self.conn, note_id, user_id).await
    }

    pub async fn delete(&self, user_id: i64, tag: DeleteTagQuery) -> Result<u64> {
        delete::delete(self.conn, user_id, &tag.name).await
    }
}