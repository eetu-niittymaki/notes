use rusqlite::{Connection, Result};

use crate::cli::TagCommand;
use crate::db::add_tag::add_tag;
use crate::db::get_all_tags::get_all_tags;
use crate::db::delete_tag::delete_tag;

pub fn tag(cmd: TagCommand, conn: &Connection) -> Result<()> {
    match cmd {
        TagCommand::Add { note_id, name } => {
            let rows = add_tag(conn, note_id, &name)?;

            if rows == 1 {
                println!("Tag '{}' added to note {}!", name, note_id);
            } else {
                println!("Note already has that tag");
            }
        }

        TagCommand::Delete { name } => {
            let rows = delete_tag(conn, &name)?;

            if rows == 1 {
                println!("Tag '{}' deleted!", name);
            } else {
                println!("No matching tag found");
            }
        }

        TagCommand::List => {
            let tags = get_all_tags(conn)?;

            if tags.is_empty() {
                println!("No tags found");
                return Ok(());
            }

            println!("All Tags:");
            println!("---------");

            for tag in tags {
                println!("{} ({} notes)", tag.name, tag.note_count);
            }
        }
    }

    Ok(())
}
