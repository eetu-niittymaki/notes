use std::collections::HashMap;

use crate::models::{Note, Tag};

pub fn build_txt(
    notes: Vec<Note>,
    note_tags: HashMap<i64, Vec<Tag>>,
) -> String {
    let mut txt = String::from("");

    for note in notes {
        let date = note.created_at.split_whitespace().next().unwrap();

        txt.push_str(&format!("{}\n", date));
        txt.push_str(&format!("{}\n", note.title));

        if let Some(tags) = note_tags.get(&note.id) {
            for tag in tags {
                txt.push_str(&format!("#{}\n", tag.name));
            }
        }
        
        txt.push_str(&format!("{}\n", note.content));
        txt.push('\n');
    }

    txt
}