use crate::error::Result;
use crate::db::Database;

use crate::utils::import::md_to_text::md_to_text;
use crate::utils::import::html_to_text::html_to_text;
use crate::utils::get_user_input::get_user_input;

pub fn import_without_separating (
    db: &Database,
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
        "md" => db.notes().create(title, &md_to_text(&content)),
        "html" => db.notes().create(title,  &html_to_text(&content)),
        "txt" => db.notes().create(title, &content),
        _ => unreachable!()
    };

    Ok(rows.unwrap())
}