#[derive(Debug)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
}

#[derive(Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub note_count: i64,
}