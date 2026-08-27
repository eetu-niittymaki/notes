use notes_core::error::Result;
use notes_core::models::note::NoteQuery;
use notes_core::models::tag::TagQuery;

use crate::client::ApiClient;
use crate::models::cli::GetCommand;


pub async fn get(cmd: GetCommand, api: &ApiClient) -> Result<()> {
    let note =  api.get_note(NoteQuery { id: cmd.id }).await?;

    let tags = api.get_tags_for_note(TagQuery { note_id: note.id}).await?;

    println!("{} | {}", note.id, note.title);
    
    if !tags.is_empty() {
        for tag in tags {
            println!("  #{}", tag.name);
        }
    }

    println!("{}", note.content);

    Ok(())  
}