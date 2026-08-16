use crate::error::Result;

use crate::cli::SearchCommand;

use crate::db::Database;

pub fn search(cmd: SearchCommand, db: &Database,) -> Result<()> {
    if cmd.title.is_some() && cmd.content.is_some() {
        eprintln!("Please provide either a title or text content, not both.");
        return Ok(());
    }

    let notes = match (&cmd.title, &cmd.content, &cmd.tag) {
        (_, _, Some(tag)) => {
            db.search().tags(tag)?
        }

        (Some(title), None, None) => {
            db.search().notes(Some(title), None)?
        }

        (None, Some(content), None) => {
            db.search().notes(None, Some(content))?
        }

        _ => {
            return Ok(());
        }
    };

    if notes.is_empty() {
        println!("No matching notes found");
        std::process::exit(0);
    }

    println!("Found Notes:");
    println!("-----------");

    for note in notes {
        println!("{} | {}\n{}", note.id, note.title, note.content);
    }

    Ok(())
}