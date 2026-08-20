use notes_core::error::Result;
use notes_core::db::Database;
use notes_core::models::note::DeleteNote;

use crate::models::cli::DeleteCommand;

use crate::utils::get_user_input::get_user_input;

use crate::config;

pub async fn delete(cmd: DeleteCommand, db: &Database,) -> Result<()> {
    let client = reqwest::Client::new();

    if cmd.all {
        println!("Delete all notes? y/n");
        let confirm = get_user_input().trim().to_lowercase();

        if confirm == "y" || confirm == "yes" {
            let response = &client
                .delete(format!("{}notes/all", config::URL))
                .send()
                .await?;

            if response.status().is_success() {
                println!("All notes deleted!");
            } else {
                println!("No notes to delete")
            }

            return Ok(())
        } else {
            println!("Note deletion stopped");
            std::process::exit(0);
        }
    } 

    let query = DeleteNote { id: cmd.id };

    let response = client
        .delete(format!("{}notes", config::URL))
        .query(&query)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Note deleted!") 
    } else  {
        println!("No matching note found")
    }
        
    Ok(())
}