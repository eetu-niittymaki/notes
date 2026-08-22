use notes_core::error::Result;
use notes_core::models::note::GetAllNotesQuery;

use crate::client::ApiClient;
use crate::models::cli::AllCommand;

pub async fn all(cmd: AllCommand, api: &ApiClient) -> Result<()> {
    let notes_with_tags = api.get_all_notes(GetAllNotesQuery { user_id: 1}).await?;

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