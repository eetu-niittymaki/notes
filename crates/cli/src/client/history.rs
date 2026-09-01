use super::ApiClient;

use notes_core::error::Result;
use notes_core::models::history::{
    NoteHistory,
    GetHistoryQuery, 
    GetVersionQuery,
    RestoreNoteQuery,
};

impl ApiClient {
    pub async fn get_full_history(&self, query: GetHistoryQuery) -> Result<Vec<NoteHistory>> {
        let response = self
            .send(self.get("notes/history/all")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_version(&self, query: GetVersionQuery) -> Result<Option<NoteHistory>> {
        let response = self
            .send(self.get("notes/history")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }

    pub async fn restore_version(&self, query: RestoreNoteQuery) -> Result<u64> {
        let response = self
            .send(self.patch("notes/history")
            .query(&query))
            .await?;
        
        Ok(response.json::<u64>().await?)
    }
}