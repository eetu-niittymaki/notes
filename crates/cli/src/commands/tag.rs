use notes_core::error::Result;
use notes_core::models::tag::{
    CreateTagQuery, 
    DeleteTagQuery
};

use crate::client::ApiClient;
use crate::models::cli::TagCommand;

pub async fn tag(cmd: TagCommand, api: &ApiClient) -> Result<()> {
    match cmd {
        TagCommand::Add { note_id, name } => {
            let add_tag = api.add_tag(
         CreateTagQuery { 
                    note_id,
                    name: name.clone()
                }
            ).await?;
            
            if add_tag > 0 {
                println!("Tag '{}' added to note", name);
            } else {
                println!("Adding tag failed!");
            }
        }

        TagCommand::Delete { name } => {
            let delete = api.delete_tag(
                DeleteTagQuery { name: name.clone() }
            ).await?;

            if delete {
                println!("Tag '{name}' deleted!");
            } else {
                println!("No matching tag found");
            }
        }

        TagCommand::List => {
            let tags = api.get_all_tags().await?;

            if tags.is_empty() {
                println!("No tags found");
                return Ok(());
            }

            println!("All Tags:");
            println!("---------");

            for tag in tags {
                let word = if tag.note_count > 1 { "notes" } else { "note" };
                println!("{} ({} {})", tag.name, tag.note_count, word);
            }
        }
    }

    Ok(())
}
