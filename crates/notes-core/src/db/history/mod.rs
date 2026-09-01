mod get;
mod create;
mod update;

use libsql::Connection;

use crate::error::Result;
use crate::models::history::{
    NoteHistory
};

pub struct HistoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> HistoryRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }   
    
    pub async fn create(
        &self, 
        user_id: i64, 
        note_id: i64, 
        version_number: i64, 
        title: &str, 
        content: &str, 
    ) -> Result<u64> {
        create::create(
            self.conn, 
            user_id, 
            note_id, 
            version_number, 
            &title, 
            &content
        ).await
    }
    
    pub async fn get_one(&self, user_id: i64, note_id: i64, version_number: i64) -> Result<NoteHistory> {
        get::one(self.conn, user_id, note_id, version_number).await
    }

    pub async fn get_all(&self, user_id: i64, id: i64) -> Result<Vec<NoteHistory>> {
        get::all(self.conn, user_id, id).await
    }

    pub async fn newest(&self, user_id: i64, id: i64) -> Result<NoteHistory> {
        get::newest(self.conn, user_id, id).await
    }

    pub async fn restore(
            &self, 
            user_id: i64, 
            note_id: i64, 
            new_title: &str, 
            new_content: &str
        ) -> Result<u64> {
        update::restore_version(self.conn, user_id, note_id, new_title, new_content).await
    }
}