use crate::error::Result;

use crate::cli::NewCommand;

use crate::db::Database;

pub fn new(cmd: NewCommand, db: &Database,) -> Result<()> {
    let rows = db.notes().create(cmd.title, &cmd.content)?;

    if rows == 1 {
        println!("Note added.");
    } else {
        eprintln!("Error in adding note");
    }

    Ok(())
}