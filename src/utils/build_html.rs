use std::collections::HashMap;

use crate::models::{Note, Tag};

pub fn build_html(
    notes: Vec<Note>,
    note_tags: HashMap<i64, Vec<Tag>>,
) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>Notes</title>
<style>
    body {
        font-family: Arial, sans-serif;
        margin: 40px;
        background-color: white;
        color: black;
    }
    .note {
        margin-bottom: 20px;
    }
    .title {
        text-decoration: underline;
        font-weight: bold;
    }
    .content {
        white-space: pre-wrap;
        margin-bottom: 10px;
    }
    .created_at {}
    .tag {
        display: block;
    }
        
</style>
</head>
<body>
"#,
    );

    for note in notes {
        html.push_str(&format!(
            r#"
<div class="note">
    <div class="title">
        <h1>{}</h1>
    </div>
    <h4 class="created_at">{}</h4>
    
"#,
        note.created_at.split_whitespace().next().unwrap(),
        note.title
    ));

    if let Some(tags) = note_tags.get(&note.id) {
        if !tags.is_empty() {
            html.push_str(r#"    <div class="tags">"#);

            for tag in tags {
                html.push_str(&format!(
                    r#"<span class="tag">#{}</span> "#,
                    tag.name
                ));
                
            }

            html.push_str("     </div>\n");
        }
    }

    html.push_str(&format!(
        r#"    <div class="content">
<p>{}</p>
</div>
</div>
"#,
            note.content
        ));
    }

    html.push_str("</body>\n</html>");

    html
}