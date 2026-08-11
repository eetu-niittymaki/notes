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
}

#[derive(Debug)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub note_count: i64,
}

pub enum NoteSelector<'a> {
    Id(i64),
    Title(&'a str),
}

pub enum NoteUpdate {
    Title(String),
    Content(String),
}