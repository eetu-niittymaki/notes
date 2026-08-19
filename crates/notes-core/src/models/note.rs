use serde::{Deserialize, Serialize};

use crate::models::tag::Tag;

#[derive(Debug, Serialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
}

#[derive( Debug, Serialize, Deserialize)]
pub struct NoteWithTags {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String
}

#[derive(Clone)]
pub enum NoteSelector<'a> {
    Id(i64),
    Title(&'a str),
}

pub enum NoteUpdate {
    Title(String),
    Content(String),
}

#[derive(Deserialize)]
pub struct NoteQuery {
    pub id: Option<i64>,
    pub title: Option<String>,
}
