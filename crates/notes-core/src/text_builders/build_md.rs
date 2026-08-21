use crate::models::note::NoteWithTags;

pub fn build_md(
    notes: Vec<NoteWithTags>,
) -> String {
    let mut md = String::from("");

    for note in notes {
        let date = note.created_at.split_whitespace().next().unwrap();

        md.push_str(&format!("# {}\n", note.title));
        md.push_str(&format!("### {}\n", date));

        if !note.tags.is_empty() {
            for tag in note.tags {
                md.push_str(&format!("* #{}\n", tag.name));
            }
        }
        
        md.push_str(&format!("## {}\n", note.content));
        md.push('\n');
    }

    md
}