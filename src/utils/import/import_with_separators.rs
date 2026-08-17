use crate::db::Database;
use crate::error::Result;

use crate::utils::import::md_to_text::md_to_text;
use crate::utils::import::html_to_text::html_to_text;
use crate::utils::text_editor::text_editor;

pub async fn import_with_separators (
    db: &Database,
    extension: &str,
    content: String,
) -> Result<u64> {
    let content = match extension {
        "md" => md_to_text(&content),
        "html" => html_to_text(&content),
        "txt" => content,
        _ => unreachable!()
    };

    let edited_content = text_editor(
        content, 
        Some(
            "Add Separator # At Title Of Each Note".to_string()
        )
    );

    let mut current_title: Option<String> = None;
    let mut current_content: Vec<String> = Vec::new();
    let mut added_notes = 0;

    for line in edited_content.lines() {
        if let Some(title) = line.strip_prefix('#') {
            if let Some(title) = current_title.take() {
                db.notes().create(title, &current_content.join("\n")).await?;
                added_notes += 1;
                current_content.clear();
            }

            current_title = Some(title.trim().to_string());
        } else if current_title.is_some() {
            current_content.push(line.to_string());
        }
    }

    if let Some(title) = current_title {
        added_notes += 1;
        db.notes().create(title, &current_content.join("\n")).await?;
    }

    Ok(added_notes)
}