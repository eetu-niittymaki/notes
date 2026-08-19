use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub note_count: i64,
}

pub struct CreateTag<'a> {
    pub id: i64,
    pub name: &'a str,
}

pub struct DeleteTag<'a> {
    pub name: &'a str,
}