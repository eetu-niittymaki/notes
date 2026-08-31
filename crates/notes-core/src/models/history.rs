use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoteHistory {
    pub id: i64,
    pub note_id: i64,
    pub version_number: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetHistoryQuery {
    pub note_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetVersionQuery {
    pub note_id: i64,
    pub version_number: i64
}

#[derive(Serialize, Deserialize)]
pub struct GetDifferenceQuery {
    pub note_id: i64,
    pub first_item: i64,
    pub second_item: i64,
}