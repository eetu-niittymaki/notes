use keyring::Entry;

const SERVICE: &str = "notes-cli";
const ACCOUNT: &str = "access-token";

pub fn load_token() -> Result<Option<String>, keyring::Error> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;

    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn save_token(token: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    entry.set_password(token)
}