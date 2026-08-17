use notes_core::error::Result;
use notes_core::db::Database;

use crate::models::cli::NewCommand;

pub async fn new(cmd: NewCommand, db: &Database,) -> Result<()> {
    let rows = db.notes().create(cmd.title, &cmd.content).await?;

    if rows == 1 {
        println!("Note added.");
    } else {
        eprintln!("Error in adding note");
    }

    Ok(())
}