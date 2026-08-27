use super::ApiClient;

use notes_core::error::Result;
use notes_core::models::tag::TagSearchQuery;
use notes_core::models::note::{Note, NoteSearchQuery};

impl ApiClient {
    pub async fn search_tags(&self, query: TagSearchQuery) -> Result<Vec<Note>> {
        let response = self.send(self
            .get("search/tags")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }

    pub async fn search_notes(&self, query: NoteSearchQuery) -> Result<Vec<Note>> {
        let response = self
        .send(self
            .get("search/notes")
            .query(&query))
            .await?;

        Ok(response.json().await?)
    }
}