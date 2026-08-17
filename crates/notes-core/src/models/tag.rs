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