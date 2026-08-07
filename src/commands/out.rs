use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use hyper_render::{render, Config, OutputFormat};
use rusqlite::{Connection, Result};

use crate::cli::OutCommand;

pub fn out(cmd: OutCommand, conn: &Connection) -> Result<()> {
    let supported_filetypes = ["md", "txt", "html", "png", "pdf"];

    if !supported_filetypes.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {:?}",
            supported_filetypes
        );
        std::process::exit(1);
    }

    let mut statement = conn.prepare("SELECT id, note FROM notes")?;

    let rows: Vec<(i64, String)> = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
            ))
        })?
        .collect::<Result<_, _>>()?;

    if rows.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(1);
    }

    let mut outfile_path = PathBuf::from(
        std::env::var("USERPROFILE").expect("USERPROFILE is not set"),
    );
    outfile_path.push("Desktop");
    outfile_path.push(format!("notes.{}", cmd.filetype));

    match cmd.filetype.as_str() {
        "txt" | "md" | "html" => export_text(&cmd, &rows, outfile_path),
        "png" | "pdf" => export_image(&cmd, &rows, outfile_path),
        _ => unreachable!(),
    }

    Ok(())
}

fn export_text(cmd: &OutCommand, rows: &[(i64, String)], outfile_path: PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile_path)
        .unwrap();

    match cmd.filetype.as_str() {
        "txt" => {
            for (_, note) in rows {
                writeln!(file, "{note}").unwrap();
            }
        }

        "md" => {
            for (_, note) in rows {
                writeln!(file, "## {note}").unwrap();
            }
        }

        "html" => {
            write!(file, "{}", build_html(rows)).unwrap();
        }

        _ => unreachable!(),
    }
}

fn export_image(cmd: &OutCommand, rows: &[(i64, String)], outfile_path: PathBuf) {
    let html = build_html(rows);

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

fn build_html(rows: &[(i64, String)]) -> String {
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
    text-align: center;
}
.note {
    margin-bottom: 20px;
}
</style>
</head>
<body>
"#,
    );

    for (_, note) in rows {
        html.push_str(&format!("<div class=\"note\"><h2>{}</h2></div>\n", note));
    }

    html.push_str("</body>\n</html>");

    html
}