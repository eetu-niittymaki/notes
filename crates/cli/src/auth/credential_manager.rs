use keyring::Entry;
use notes_core::error::Result;
use serde::{Deserialize, Serialize};

const SERVICE: &str = "notes-app";
const ACCOUNT: &str = "authentication";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    //pub refresh_token: String,
}

fn entry() -> Result<Entry> {
    Ok(Entry::new(SERVICE, ACCOUNT)?)
}

pub async fn save_tokens(tokens: &Tokens) -> Result<()> {
    let entry = entry()?;

    let serialized = serde_json::to_string(tokens)?;

    entry.set_password(&serialized)?;

    Ok(())
}

pub async fn load_tokens() -> Result<Option<Tokens>> {
    let entry = entry()?;

    match entry.get_password() {
        Ok(value) => {
            let tokens = serde_json::from_str(&value)?;
            Ok(Some(tokens))
        }

        Err(keyring::Error::NoEntry) => Ok(None),

        Err(error) => Err(error.into()),
    }
}

pub fn delete_tokens() -> Result<()> {
    let entry = entry()?;

    match entry.delete_credential() {
        Ok(_) => Ok(()),

        Err(keyring::Error::NoEntry) => Ok(()),

        Err(error) => Err(error.into()),
    }
}