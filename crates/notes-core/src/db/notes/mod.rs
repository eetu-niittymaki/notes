mod get;
mod create;
mod update;
mod delete;

use libsql::Connection;

use crate::error::Result;

use crate::models::note::{
    CreateNote, Note, NoteSelector, NoteUpdate, NoteWithTags
};

pub struct NotesRepository<'a> {
    conn: &'a Connection,
}

impl<'a> NotesRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn create(&self, note: CreateNote) -> Result<u64> {
        create::create(self.conn, &note.title, &note.content).await
    }
    
    pub async fn get(&self, selector: NoteSelector<'_>) -> Result<Note> {
        get::one(self.conn, &selector).await
    }

    pub async fn get_all(&self) -> Result<Vec<Note>> {
        get::all(self.conn).await
    }

    pub async fn get_all_with_tags(&self) -> Result<Vec<NoteWithTags>> {
        get::all_with_tags(self.conn).await
    }

    pub async fn update(&self, selector: NoteSelector<'_>, updater: NoteUpdate) -> Result<u64> {
        update::update(self.conn, &selector, updater).await
    }

    pub async fn delete(&self, selector: NoteSelector<'_>) -> Result<u64> {
        delete::delete(self.conn, &selector).await
    }

    pub async fn delete_all(&self) -> Result<u64> {
        delete::delete_all(self.conn).await
    }
}