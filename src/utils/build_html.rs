use crate::models::Note;

pub fn build_html(notes: Vec<Note>) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
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
}
.content {
    margin-bottom: 10px;
}
.created_at {
    font-weight: bold;
}
    
</style>
</head>
<body>
"#,
    );

    for note in notes {
        html.push_str(&format!(
"<div class=\"note\">
    <h4 class=\"created_at\">{}</h4>
    <div class=\"title\">
        <h2>{}</h2>
    </div> 
    <div class=\"content\">{}</div>
</div>\n", 
            note.created_at.split_whitespace().next().unwrap(),
            note.title, 
            note.content
            )
        );
    }

    html.push_str("</body>\n</html>");

    html
}