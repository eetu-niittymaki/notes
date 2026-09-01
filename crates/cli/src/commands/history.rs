use diffy::{create_patch, PatchFormatter};

use notes_core::error::Result;
use notes_core::models::history::{
    GetHistoryQuery, 
    GetVersionQuery, 
    NoteHistory, 
    RestoreNoteQuery
};

use crate::client::ApiClient;
use crate::models::cli::{HistoryAction, HistoryCommand};

fn print_header() {
    println!(
        "{:<8} | {:<20} | {:<30} | {:<20} | {}",
        "Version",
        "Title",
        "Content",
        "Operation",
        "Date"
    );

    println!(
        "{:-<8}-+-{:-<20}-+-{:-<30}-+-{:-<20}-+-{:-<20}",
        "",
        "",
        "",
        "",
        ""
    );
}

fn print_row(note: &NoteHistory) {
    println!(
        "{:<8} | {:<20} | {:<30} | {:<20} | {}",
        format!("v{}", note.version_number),
        note.title,
        note.content,
        note.operation,
        note.created_at
    );
}

pub async fn history(cmd: HistoryCommand, api: &ApiClient) -> Result<()> {
    let note_id = cmd.note_id;

    match cmd.action {
        HistoryAction::All => {
            let notes: Vec<NoteHistory> = api.get_full_history(
                GetHistoryQuery { note_id }
            ).await?;

            if notes.is_empty() {
                println!("Note not found");
                return Ok(())
            }

            print_header();
            
            for note in notes {
                print_row(&note);
            }
        }

        HistoryAction::Get { version_number } => {
            let note = api.get_version( 
                GetVersionQuery { note_id, version_number }
            ).await?;

            if let Some(note) = note {
                print_header();
                print_row(&note)
            } 
        }

        HistoryAction::Diff { first_item, second_item } => {
            let version_one = api.get_version( 
                GetVersionQuery { note_id, version_number: first_item }
            ).await?;

            let version_two = api.get_version( 
                GetVersionQuery { note_id, version_number: second_item }
            ).await?;

            if let Some(version_one) = version_one &&
                let Some(version_two) = version_two {
                    let title_patch = create_patch(
                        &version_one.title, 
                        &version_two.title
                    );

                    let content_patch = create_patch(
                        &version_one.content, 
                        &version_two.content
                    );
                    
                    let f = PatchFormatter::new().with_color();
                    println!("Title: \n{}", f.fmt_patch(&title_patch));
                    print!("Content: \n{}", f.fmt_patch(&content_patch));
                }
        }

        HistoryAction::Restore { version_number } => {
            let version = api.get_version(
                GetVersionQuery { note_id, version_number }
            ).await?;

            if let Some(version) = version {
                let query = RestoreNoteQuery { 
                    note_id: version.note_id,
                    title: version.title,
                    content: version.content,
                };

                let restore = api.restore_version(query).await?;

                if restore  > 0 {
                    println!("Note restored to v{}", version.version_number);
                } else {
                    println!("Note restoration failed")
                }
            }
        }
    }

    Ok(())
}