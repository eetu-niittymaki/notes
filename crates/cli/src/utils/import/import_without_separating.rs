use crate::client::ApiClient;
use notes_core::error::Result;
use notes_core::models::note::CreateNoteQuery;
use notes_core::text_parsers::md_to_text::md_to_text;
use notes_core::text_parsers::html_to_text::html_to_text;
use crate::utils::get_user_input::get_user_input;

pub async fn import_without_separating (
    api: &ApiClient,
    extension: &str,
    content: String,
    title: &str
) -> Result<u64> {
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
            return Ok(0)
        }
    };

    let changed_rows = match extension {
        "md" => api.create_note(CreateNoteQuery { 
            title: title, 
            content: md_to_text(&content) }).await?,

        "html" => api.create_note(CreateNoteQuery { 
            title: title,  
            content: html_to_text(&content) }).await?,

        "txt" => api.create_note(CreateNoteQuery { 
            title: title, 
            content: content }).await?,
        _ => unreachable!()
    };

    Ok(changed_rows)
}