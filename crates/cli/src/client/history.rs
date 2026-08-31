use super::ApiClient;

use notes_core::error::Result;
use notes_core::models::history::{
    GetHistoryQuery, 
    GetVersionQuery, 
    NoteHistory
};

impl ApiClient {
    pub async fn get_full_history(&self, query: GetHistoryQuery) -> Result<Vec<NoteHistory>> {
        let response = self
            .send(self.get("notes/history/all")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_version(&self, query: GetVersionQuery) -> Result<NoteHistory> {
        let response = self
            .send(self.get("notes/history")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }
}