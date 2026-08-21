use notes_core::error::Result;
use notes_core::models::note::NoteSearchQuery;
use notes_core::models::tag::TagSearchQuery;

use crate::client::ApiClient;
use crate::models::cli::SearchCommand;

pub async fn search(cmd: SearchCommand, api: &ApiClient,) -> Result<()> {
    if cmd.title.is_some() && cmd.content.is_some() {
        eprintln!("Please provide either a title or text content, not both.");
        return Ok(());
    }

    let notes = match (&cmd.title, &cmd.content, &cmd.tag) {
        (_, _, Some(tag)) => {
            api.search_tags(TagSearchQuery {
                tag: tag.to_string(),
            }).await?
        }

        (title, content, None) if title.is_some() || content.is_some() => {
            api.search_notes(NoteSearchQuery {
                title: title.clone(),
                content: content.clone(),
            }).await?
        }

        _ => {
            return Ok(());
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