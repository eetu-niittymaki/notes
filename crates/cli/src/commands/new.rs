use notes_core::error::Result;
use notes_core::db::Database;

use notes_core::models::note::CreateNote;

use crate::models::cli::NewCommand;

pub async fn new(cmd: NewCommand, db: &Database,) -> Result<()> {
    let rows = db.notes().create(CreateNote {
        title: cmd.title, 
        content: cmd.content
    }).await?;

    if rows == 1 {
        println!("Note added.");
    } else {
        eprintln!("Error in adding note");
    }

    Ok(())
}