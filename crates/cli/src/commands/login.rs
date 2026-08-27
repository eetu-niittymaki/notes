use notes_core::error::Result;

use crate::client::ApiClient;
use crate::auth::credential_manager;
use crate::auth::auth;

pub async fn login(api: &ApiClient) -> Result<()> {
    match credential_manager::load_tokens().await? {
        Some(_token) => { 
            println!("Already logged in");
            Ok(())
        }
        None => {
            auth::login(api).await?;
            Ok(())
        }
    }
}