use super::ApiClient;

use notes_core::error::Result;
use notes_core::models::tag::{
    Tag,
    TagWithCount,
    TagQuery,
    CreateTagQuery,
    DeleteTagQuery
};

impl ApiClient {
    pub async fn get_all_tags(&self) -> Result<Vec<TagWithCount>> {
        let response = self
            .send(self
                .get("tags/all"))
                .await?;

        Ok(response.json().await?)
    }

    pub async fn get_tags_for_note(&self, query: TagQuery) -> Result<Vec<Tag>> {
        let response = self
            .send(self
                .get("tags")
                .query(&query))
                .await?;

        Ok(response.json().await?)
    }

    pub async fn add_tag(&self, query: CreateTagQuery) -> Result<u64> {
        let response = self
            .send(self
                .post("tags")
                .query(&query))
                .await?;

        Ok(response.json().await?)    
    }

    pub async fn delete_tag(&self, query: DeleteTagQuery) -> Result<bool> {
        let response = self
            .send(self
                .delete("tags")
                .query(&query))
                .await?;

         Ok(response.status().is_success()) 
    }
}