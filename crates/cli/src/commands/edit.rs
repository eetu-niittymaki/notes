use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::{
    Note, 
    NoteUpdate, 
    NoteWithTags, 
    UpdateNoteQuery
};

use crate::config;
use crate::models::cli::{EditCommand, EditField};
use crate::utils::text_editor::text_editor;

pub async fn edit(cmd: EditCommand, db: &Database,) -> Result<()> {
    let client = reqwest::Client::new();

    let note = client
        .get(format!("{}note", config::URL))
        .query(&[("id", cmd.id)])
        .send()
        .await?
        .json::<NoteWithTags>()
        .await?;

    let query = match cmd.field {
        EditField::Title => {
            let new_title = text_editor(
                note.title,
                Some("Edit Title".to_string()),
            );

            UpdateNoteQuery {
                id: cmd.id,
                title: Some(new_title),
                content: None,
            }
        }

        EditField::Content => {
            let new_content = text_editor(
                note.content,
                Some("Edit Content".to_string()),
            );

            UpdateNoteQuery {
                id: cmd.id,
                title: None,
                content: Some(new_content),
            }
        }
    };

    let response = client
        .patch(format!("{}notes", config::URL))
        .query(&query)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Note updated successfully!");
    }

    Ok(())
}