use notes_core::error::Result;
use notes_core::models::history::{GetHistoryQuery, GetVersionQuery};

use crate::client::ApiClient;
use crate::models::cli::HistoryCommand;

pub async fn history(cmd: HistoryCommand, api: &ApiClient) -> Result<()> {
    match cmd {
        HistoryCommand::All { note_id } => {
            let notes: Vec<notes_core::models::history::NoteHistory> = api.get_full_history(
                GetHistoryQuery { note_id }
            ).await?;

            println!("Version | Title | Content | Date");
            println!("--------------------------------");
            
            for note in notes {
                println!("v{} | {} | {} | {}", note.version_number, note.title, note.content, note.created_at)
            }
        }

        HistoryCommand::Get { note_id, version_number } => {
            let note = api.get_version( 
                GetVersionQuery { note_id, version_number }
            ).await?;

            println!("Version | Title | Content | Date");
            println!("--------------------------------");
            println!("v{} | {} | {} | {}", note.version_number, note.title, note.content, note.created_at)
        }
    }

    Ok(())
}