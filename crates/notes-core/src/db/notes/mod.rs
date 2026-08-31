mod get;
mod create;
mod update;
mod delete;

use libsql::Connection;

use crate::error::Result;

use crate::models::note::{
    CreateNoteQuery, 
    Note, 
    NoteUpdate, 
    NoteWithTags
};

pub struct NotesRepository<'a> {
    conn: &'a Connection,
}

impl<'a> NotesRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn create(&self, user_id: i64, note: CreateNoteQuery) -> Result<i64> {
        create::create(self.conn, user_id, &note.title, &note.content).await
    }
    
    pub async fn get(&self, user_id: i64, id: i64) -> Result<NoteWithTags> {
        get::one(self.conn, user_id, id).await
    }

    pub async fn get_all(&self, user_id: i64) -> Result<Vec<Note>> {
        get::all(self.conn, user_id).await
    }

    pub async fn get_all_with_tags(&self, user_id: i64) -> Result<Vec<NoteWithTags>> {
        get::all_with_tags(self.conn, user_id).await
    }

    pub async fn update(&self, user_id: i64, id: i64, updater: NoteUpdate) -> Result<u64> {
        update::update(self.conn, user_id, id, updater).await
    }

    pub async fn delete(&self, user_id: i64, id: i64) -> Result<u64> {
        delete::delete(self.conn, user_id, id).await
    }

    pub async fn delete_all(&self, user_id: i64) -> Result<u64> {
        delete::delete_all(self.conn, user_id).await
    }
}