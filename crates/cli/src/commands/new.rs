use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::CreateNote;

use crate::config;
use crate::models::cli::NewCommand;

pub async fn new(cmd: NewCommand, db: &Database,) -> Result<()> {
    let client = reqwest::Client::new();
    let body = CreateNote {
        title: cmd.title, 
        content: cmd.content
    };

    let response = client
        .post(format!("{}notes", config::URL))
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
         println!("Note added");
    } else {
        println!("Error in adding note");
    }
    
    Ok(())
}