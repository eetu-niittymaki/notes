use notes_core::error::Result;
use notes_core::models::note::{
    NoteQuery,
    UpdateNoteQuery
};

use crate::client::ApiClient;
use crate::models::cli::{EditCommand, EditField};
use crate::utils::text_editor::text_editor;

pub async fn edit(cmd: EditCommand, api: &ApiClient) -> Result<()> {
    let note = api.get_note(NoteQuery { id: cmd.id }).await?;

    // Shape query relatively based on if title or content command flag is given
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

    let update = api.update_note(query).await?;

    if update {
        println!("Note updated successfully");
    } else {
        println!("Error in updating note!")
    }

    Ok(())
}