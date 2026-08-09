use rusqlite::{Connection, Result};

use crate::cli::TagCommand;
use crate::db::add_tag::add_tag;
use crate::db::get_all_tags::get_all_tags;

pub fn tag(cmd: TagCommand, conn: &Connection) -> Result <()> {
    if cmd.list {
        let tags = get_all_tags(conn)?;

        if tags.is_empty() {
            println!("No tags found");
            std::process::exit(0);
        }

        println!("All Tags:");
        println!("----------");

        for tag in tags {
            println!("{}", tag.name);
        }

        return Ok(())
    }

    let add_tag = add_tag(conn, cmd.id, cmd.tag.as_deref())?;
    
    if add_tag == 1 {
        println!("Tag: {:#?} added to note!", &cmd.tag);
    } else {
        println!("Note already has that tag")
    }

    Ok(())
}