use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use hyper_render::{render, Config, OutputFormat};
use rusqlite::{Connection, Result};

use crate::cli::OutCommand;
use crate::db::get_all_notes::get_all_notes;
use crate::models::Note;

pub fn out(cmd: OutCommand, conn: &Connection) -> Result<()> {
    let supported_filetypes = ["md", "txt", "html", "png", "pdf"];

    if !supported_filetypes.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {:?}",
            supported_filetypes
        );
        std::process::exit(1);
    }

    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(1);
    }

    let mut outfile_path = PathBuf::from(
        std::env::var("USERPROFILE").expect("USERPROFILE is not set"),
    );
    outfile_path.push("Desktop");
    outfile_path.push(format!("notes.{}", cmd.filetype));

    match cmd.filetype.as_str() {
        "txt" | "md" | "html" => export_text(&cmd, notes, outfile_path),
        "png" | "pdf" => export_image(&cmd, notes, outfile_path),
        _ => unreachable!(),
    }

    Ok(())
}

fn export_text(cmd: &OutCommand, notes: Vec<Note> , outfile_path: PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile_path)
        .unwrap();

    match cmd.filetype.as_str() {
        "txt" => {
            for note in notes {
                writeln!(file, "{}: {}", note.title, note.content).unwrap();
            }
        }

        "md" => {
            for note in notes {
                writeln!(file, "## {}: {}", note.title, note.content).unwrap();
            }
        }

        "html" => {
            write!(file, "{}", build_html(notes)).unwrap();
        }

        _ => unreachable!(),
    }
}

fn export_image(cmd: &OutCommand, notes: Vec<Note>, outfile_path: PathBuf) {
    let html = build_html(notes);

    let config = Config::default().format(match cmd.filetype.as_str() {
        "png" => OutputFormat::Png,
        "pdf" => OutputFormat::Pdf,
        _ => unreachable!(),
    });

    let bytes = match render(&html, config) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Render failed: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = std::fs::write(outfile_path, bytes) {
        eprintln!("Failed to write output: {e}");
        std::process::exit(1);
    }
}

fn build_html(notes: Vec<Note>) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Notes</title>
<style>
body {
    font-family: Arial, sans-serif;
    margin: 40px;
    background-color: white;
    color: black;
}
.note {
    margin-bottom: 20px;
}
.title {
    text-decoration: underline;
}
.content {
    margin-bottom: 10px;
}
.created_at {
    font-weight: bold;
}
    
</style>
</head>
<body>
"#,
    );

    for note in notes {
        html.push_str(&format!(
"<div class=\"note\">
    <h4 class=\"created_at\">{}</h4>
    <div class=\"title\">
        <h2>{}</h2>
    </div> 
    <div class=\"content\">{}</div>
</div>\n", 
            note.created_at.split_whitespace().next().unwrap(),
            note.title, 
            note.content
            )
        );
    }

    html.push_str("</body>\n</html>");

    html
}