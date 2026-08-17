use crate::error::Result;

use crate::models::cli::DeleteCommand;
use crate::models::note::NoteSelector;

use crate::db::Database;

use crate::utils::get_user_input::get_user_input;



pub async fn delete(cmd: DeleteCommand, db: &Database,) -> Result<()> {
    if cmd.all {
        println!("Delete all notes? y/n");
        let confirm = get_user_input().trim().to_lowercase();

        if confirm == "y" || confirm == "yes" {
            let rows = db.notes().delete_all().await?;
            if rows > 0 {
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

    let selector = match (cmd.id, cmd.title.as_deref()) {
        (Some(id), None) => NoteSelector::Id(id),

        (None, Some(title)) => NoteSelector::Title(title),

        (None, None) => {
            eprintln!("Please provide either --id or --title");
            return Ok(());
        }

        (Some(_), Some(_)) => {
            eprintln!("Please provide either --id or --title, not both");
            return Ok(());
        }
    };
    
    let rows = db.notes().delete(selector).await?;
    if rows == 1 {
        println!("Note deleted!") 
    } else  {
        println!("No matching note found")
    }
        


    Ok(())
}