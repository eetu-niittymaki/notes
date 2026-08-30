use super::ApiClient;

use notes_core::error::{Result, Error};
use notes_core::models::note::{
    CreateNoteQuery, 
    DeleteNoteQuery, 
    NoteQuery, 
    NoteWithTags, 
    UpdateNoteQuery
};

impl ApiClient {
    pub async fn get_all_notes(&self) -> Result<Vec<NoteWithTags>> {
        let response = self
            .send(self.get("notes/all"))
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_note(&self, query: NoteQuery) -> Result<NoteWithTags, Error> {
        let response = self
            .send(self
            .get("notes")
            .query(&query))
            .await?;

        Ok(response.json::<NoteWithTags>().await?)  
    }

    pub async fn create_note(&self, query: CreateNoteQuery) -> Result<u64> {
        let response = self
            .send(self
                .post("notes")
                .json(&query))
                .await?;

        Ok(response.json::<u64>().await?)     
    }

    pub async fn update_note(&self, query: UpdateNoteQuery) -> Result<bool, Error> {
        self.send(self
            .patch("notes")
            .query(&query))
            .await?;
            
        Ok(true)
    }

    pub async fn delete_note(&self, query: DeleteNoteQuery) -> Result<bool, Error> {
        self.send(self
            .delete("notes")
            .query(&query))
            .await?;
        
        Ok(true)
    }

    pub async fn delete_all_notes(&self) -> Result<bool, Error> {
        self.send(self
            .delete("notes/all"))
            .await?;

        Ok(true)
    }
}