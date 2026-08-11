use std::collections::HashMap;

use crate::models::{Note, Tag};

pub fn build_json(notes: Vec<Note>, note_tags: HashMap<i64, Vec<Tag>>) -> String {
  let mut json = String::from("{\n  \"notes\": [\n");

  for (note_index, note) in notes.iter().enumerate() {
    let date = note.created_at.split_whitespace().next().unwrap();

    json.push_str("    {\n");
    json.push_str(&format!(
      "      \"id\": {},\n",
      note.id
    ));

    json.push_str(&format!(
      "      \"created_at\": \"{}\",\n",
      date
    ));

    json.push_str(&format!(
      "      \"title\": \"{}\",\n",
      note.title
    ));

    if let Some(tags) = note_tags.get(&note.id) {
      if !tags.is_empty() {
        json.push_str("      \"tags\": [\n");

        for (tag_index, tag) in tags.iter().enumerate() {
          json.push_str("        {\n");
          json.push_str(&format!(
            "          \"id\": {},\n",
            tag.id
          ));
          json.push_str(&format!(
            "          \"name\": \"{}\"\n",
            tag.name
          ));

          if tag_index + 1 < tags.len() {
            json.push_str("        },\n");
          } else {
            json.push_str("        }\n");
          }
        }

        json.push_str("      ],\n");
      }
    }

    json.push_str(&format!(
      "      \"content\": \"{}\"\n",
      note.content
    ));

    if note_index + 1 < notes.len() {
      json.push_str("    },\n");
    } else {
      json.push_str("    }\n");
    }
  }

  json.push_str("  ]\n");
  json.push('}');

  json
}
