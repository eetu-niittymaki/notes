use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::CreateNote;

use crate::models::cli::NewCommand;

pub async fn new(cmd: NewCommand, db: &Database,) -> Result<()> {
    let client = reqwest::Client::new();
    let body = CreateNote {
        title: cmd.title, 
        content: cmd.content
    };

    let response = client
        .post("http://127.0.0.1:8080/notes")
        .json(&body)
        .send()
        .await?;

    response.error_for_status()?;
    
    println!("Note added.");

    Ok(())
}