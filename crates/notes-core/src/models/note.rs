use serde::{Deserialize, Serialize};

use crate::models::tag::Tag;

#[derive(Debug, Serialize, Deserialize)]
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
pub struct CreateNoteQuery {
    pub title: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNoteQuery {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct DeleteNoteQuery {
    pub id: i64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NoteQuery {
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NoteSearchQuery {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NoteResponse {
    pub notes: Vec<Note>,
}

#[derive(Serialize, Deserialize)]
pub enum NoteUpdate {
    Title(String),
    Content(String),
}


