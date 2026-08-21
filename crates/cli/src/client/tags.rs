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
        Ok(self
            .http
            .get(format!("{}tag", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?
        )
    }

    pub async fn get_tags_for_note(&self, query: TagQuery) -> Result<Vec<Tag>> {
        Ok(self
            .http
            .get(format!("{}tags", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Tag>>()
            .await?
        )
    }

    pub async fn add_tag(&self, query: CreateTagQuery) -> Result<Tag> {
        Ok(self
            .http
            .post(format!("{}tags", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?
            .json::<Tag>()
            .await?
        )
    }

    pub async fn delete_tag(&self, query: DeleteTagQuery) -> Result<bool> {
        let response = self
            .http
            .delete(format!("{}tags", self.base_url))
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let rows_affected = response.json::<u64>().await?;

        Ok(rows_affected > 0)
    }
}