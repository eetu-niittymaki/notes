use notes_core::error::Result;

use crate::utils::get_user_input::get_user_input;
use crate::auth::credential_manager;

pub async fn logout() -> Result<()> {
    println!("Log out of this machine, y/n?");

    match get_user_input().to_lowercase().as_str() {
        "yes" | "y" => credential_manager::delete_tokens(),
        _ => std::process::exit(0)
    }
}