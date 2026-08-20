use notes_core::error::Result;
use notes_core::db::Database;

use crate::config;
use crate::models::cli::GetCommand;

use notes_core::models::note::NoteWithTags;

pub async fn get(cmd: GetCommand, db: &Database,) -> Result<()> {
    let client = reqwest::Client::new();

    let note =  client
        .get(format!("{}note", config::URL))
        .query(&[("id", cmd.id)])
        .send()
        .await?
        .json::<NoteWithTags>()
        .await?;

    let tags = db.tags().for_note(note.id).await?;

    println!("{} | {}", note.id, note.title);
    
    if !tags.is_empty() {
        for tag in tags {
            println!("  #{}", tag.name);
        }
    }

    println!("{}", note.content);

    Ok(())  
}