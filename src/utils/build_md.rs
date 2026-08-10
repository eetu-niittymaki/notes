use std::collections::HashMap;

use crate::models::{Note, Tag};

pub fn build_md(
    notes: Vec<Note>,
    note_tags: HashMap<i64, Vec<Tag>>,
) -> String {
    let mut md = String::from("");

    for note in notes {
        let date = note.created_at.split_whitespace().next().unwrap();

        md.push_str(&format!("### {}\n", date));
        md.push_str(&format!("# {}\n", note.title));

        if let Some(tags) = note_tags.get(&note.id) {
            for tag in tags {
                md.push_str(&format!("* #{}\n", tag.name));
            }
        }
        md.push_str(&format!("## {}\n", note.content));
        md.push('\n');
    }

    md
}