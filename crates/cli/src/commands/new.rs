use notes_core::error::Result;

use notes_core::models::note::CreateNoteQuery;

use crate::client::ApiClient;
use crate::models::cli::NewCommand;

pub async fn new(cmd: NewCommand, api: &ApiClient) -> Result<()> {
    let query = CreateNoteQuery {
        title: cmd.title, 
        content: cmd.content
    };

    let create_note = api.create_note(query).await?;

    if create_note > 0 {
        println!("Note added");
    } else {
        println!("Failed to add note!");
    }
    
    Ok(())
}