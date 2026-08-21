use notes_core::error::Result;
use notes_core::models::note::DeleteNoteQuery;

use crate::client::ApiClient;
use crate::models::cli::DeleteCommand;

use crate::utils::get_user_input::get_user_input;

pub async fn delete(cmd: DeleteCommand, api: &ApiClient) -> Result<()> {

    if cmd.all {
        println!("Delete all notes? y/n");
        let confirm = get_user_input().trim().to_lowercase();

        if confirm == "y" || confirm == "yes" {
            let delete_all = api.delete_all_notes().await?;

            if delete_all {
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

    let query = DeleteNoteQuery { id: cmd.id };

    let delete = api.delete_note(query).await?;

    if delete {
        println!("Note deleted!") 
    } else  {
        println!("No matching note found")
    }
        
    Ok(())
}