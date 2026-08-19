use notes_core::db::Database;
use notes_core::error::Result;

use crate::models::cli::AllCommand;

use notes_core::models::note::NoteWithTags;

pub async fn all(cmd: AllCommand, db: &Database) -> Result<()> {
    let notes_with_tags = reqwest::get("http://127.0.0.1:8080/notes")
        .await?
        .json::<Vec<NoteWithTags>>()
        .await?;

    if notes_with_tags.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    println!("All Notes:");
    println!("----------");

    for note in notes_with_tags {
        println!("{} | {}", note.id, note.title);

        for tag in note.tags {
            println!("  #{}", tag.name);
        }

        if cmd.content {
            println!("\n{}", note.content);
        }
    } 

    Ok(())
}