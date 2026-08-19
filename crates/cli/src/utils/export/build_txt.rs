use notes_core::models::note::NoteWithTags;

pub fn build_txt(
    notes: Vec<NoteWithTags>,
) -> String {
    let mut txt = String::from("");

    for note in notes {
        let date = note.created_at.split_whitespace().next().unwrap();

        txt.push_str(&format!("{}\n", date));
        txt.push_str(&format!("{}\n", note.title));

        if !note.tags.is_empty() {
            for tag in note.tags {
                txt.push_str(&format!("#{}\n", tag.name));
            }
        }
        
        txt.push_str(&format!("{}\n", note.content));
        txt.push('\n');
    }

    txt
}