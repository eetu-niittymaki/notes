use super::ApiClient;
use notes_core::error::Result;
use notes_core::models::tag::TagSearchQuery;
use notes_core::models::note::{Note, NoteSearchQuery};

impl ApiClient {
    pub async fn search_tags(&self, query: TagSearchQuery) -> Result<Vec<Note>> {
         Ok(self
            .http
            .get(format!("{}search/tags", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Note>>()
            .await?
        )
    }

    pub async fn search_notes(&self, query: NoteSearchQuery) -> Result<Vec<Note>> {
        Ok(self
            .http
            .get(format!("{}search/notes", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Note>>()
            .await?
        )
    }
}