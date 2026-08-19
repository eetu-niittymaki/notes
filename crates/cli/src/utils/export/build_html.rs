use notes_core::models::note::NoteWithTags;

pub fn build_html(
    notes: Vec<NoteWithTags>,
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

    if !note.tags.is_empty() {
        html.push_str(r#"    <div class="tags">"#);

        for tag in note.tags {
            html.push_str(&format!(
                r#"<span class="tag">#{}</span> "#,
                tag.name
            ));
            
        }

        html.push_str("     </div>\n");
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