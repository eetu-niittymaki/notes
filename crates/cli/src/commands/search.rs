use notes_core::error::Result;
use notes_core::models::note::NoteSearchQuery;
use notes_core::models::tag::TagSearchQuery;

use crate::client::ApiClient;
use crate::models::cli::{SearchCommand, SearchField};

pub async fn search(cmd: SearchCommand, api: &ApiClient,) -> Result<()> {
    let notes = match cmd.field {
        SearchField::Tag => {
            api.search_tags(TagSearchQuery {
                tag: cmd.pattern,
            }).await?
        }

        SearchField::Title => {
            api.search_notes(NoteSearchQuery { 
                title: Some(cmd.pattern), 
                content: None
            }).await?
        }

        SearchField::Content => {
            api.search_notes(NoteSearchQuery { 
                title: None,
                content: Some(cmd.pattern)
            }).await?
        }
    };

    if notes.is_empty() {
        println!("No matching notes found");
        std::process::exit(0);
    }

    println!("Found Notes:");
    println!("-----------");

    for note in notes {
        println!("{} | {}\n{}", note.id, note.title, note.content);
    }

    Ok(())
}