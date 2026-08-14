use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::models::{Note, Tag};

pub fn build_json(notes: Vec<Note>, note_tags: HashMap<i64, Vec<Tag>>) -> String {
    let notes_json: Vec<_> = notes
        .iter()
        .map(|note| {
            let date = note.created_at.split_whitespace().next().unwrap();

            let tags: Vec<_> = note_tags
                .get(&note.id)
                .map(|tags| {
                    tags.iter()
                        .map(|tag| {
                            json!({
                                "id": tag.id,
                                "name": tag.name
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let mut note_json = Map::new();

            note_json.insert("id".into(), json!(note.id));
            note_json.insert("created".into(), json!(date));
            note_json.insert("title".into(), json!(note.title));

            if !tags.is_empty() {
                note_json.insert("tags".into(), json!(tags));
            }

            note_json.insert("content".into(), json!(note.content));

            Value::Object(note_json)
        })
        .collect();

    serde_json::to_string_pretty(&json!({
        "notes": notes_json
    }))
    .unwrap()
}
