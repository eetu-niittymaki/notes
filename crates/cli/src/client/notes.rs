use super::ApiClient;
use notes_core::error::Result;
use notes_core::models::note::{
    NoteWithTags,
    NoteQuery,
    CreateNoteQuery, 
    DeleteNoteQuery, 
    UpdateNoteQuery
};

impl ApiClient {
    pub async fn get_all_notes(&self) -> Result<Vec<NoteWithTags>> {
        Ok(self
            .http
            .get(format!("{}notes/all", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?
        )
    }

    pub async fn get_note(&self, query: NoteQuery) -> Result<NoteWithTags> {
        Ok(self
            .http
            .get(format!("{}notes", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<NoteWithTags>()
            .await?
        )
    }

    pub async fn create_note(&self, query: CreateNoteQuery) -> Result<u64> {
        Ok(self
            .http
            .post(format!("{}notes", self.base_url))
            .json(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<u64>()
            .await?
        )
    }

    pub async fn update_note(&self, query: UpdateNoteQuery) -> Result<bool> {
        let response = self
            .http
            .patch(format!("{}notes", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let rows_affected = response.json::<u64>().await?;

        Ok(rows_affected > 0)
    }

    pub async fn delete_note(&self, query: DeleteNoteQuery) -> Result<bool> {
        self
            .http
            .delete(format!("{}notes", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }

    pub async fn delete_all_notes(&self) -> Result<bool> {
        let response = self
            .http
            .delete(format!("{}notes/all", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        let rows_affected = response.json::<u64>().await?;

        Ok(rows_affected > 0)
    }
}