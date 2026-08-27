use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub note_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQuery {
    pub note_id: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTagQuery {
    pub note_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteTagQuery {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TagSearchQuery {
    pub tag: String,
}