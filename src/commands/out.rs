use std::fs::OpenOptions;
use std::io::Write;

use rusqlite::{Connection, Result};

use crate::cli::OutCommand;

pub fn out(cmd: OutCommand, conn: &Connection) -> Result<()> {
    let supported_filetypes = ["md", "txt", "html", "png"];

    if !supported_filetypes.contains(&cmd.filetype.as_str()) {
        eprintln!("Unsupported filetype, use md, txt or png!");
        std::process::exit(1);
    }

    let mut statement = conn.prepare("SELECT id, note FROM notes")?;

    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    let filename = format!("notes.{}", cmd.filetype);

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open output file: {e}");
            std::process::exit(1);
        }
    };

    let mut first_row = true;

    for row in rows {
        let (_id, note) = row?;

        if first_row {
            let write_first_row = match cmd.filetype.as_str() {
                "md" => writeln!(file, "## Notes"),
                "html" => writeln!(file, "<h1>Notes</h1>"),
                _ => writeln!(file, "Notes"),
            };

            if let Err(e) = write_first_row {
                eprintln!("Failed to write file: {e}");
                std::process::exit(1);
            }

            first_row = false;
        }

        let write_rows = match cmd.filetype.as_str() {
                "md" => writeln!(file, "### {}", note),
                "html" => writeln!(file, "<h2>{}</h2>", note),
                _ => writeln!(file, "{}", note),
            };

            if let Err(e) = write_rows {
                eprintln!("Failed to write file: {e}");
                std::process::exit(1);
            }
    }

    Ok(())
}