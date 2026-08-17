#[derive(Debug)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
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