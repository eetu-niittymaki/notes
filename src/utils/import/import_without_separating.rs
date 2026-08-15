use rusqlite::{Connection, Result};

use crate::utils::import::md_to_text::md_to_text;
use crate::utils::import::html_to_text::html_to_text;
use crate::utils::get_user_input::get_user_input;
use crate::db::add_note::add_note;

pub fn import_without_separating (
    conn: &Connection,
    extension: &str,
    content: String,
    title: &str
) -> Result<usize> {
    println!("How to set title?
[1] Use filename
[2] Give new title");

    let mode: u8 = get_user_input()
        .parse()
        .unwrap_or(0);

    let title = match mode {
        1 => title.to_string(),
        2 => {
            println!("Give title");
            get_user_input()
        },
        _ => {
            println!("Please enter a number from 1-2.");
            return Ok(0);
        }
    };

    let rows = match extension {
        "md" => {
            let text = md_to_text(&content);
            add_note(conn, &title, &text)?
        },
        "html" => {
            let text = html_to_text(&content);
            add_note(conn, &title, &text)?
        },
        "txt" => {
            add_note(conn, &title, &content)?
        }
        _ => unreachable!()
    };

    Ok(rows)
}