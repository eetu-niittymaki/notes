use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Error from SQLite/database operation.
    Database(libsql::Error),
    // Error from input/output operations,
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Keyring(keyring::Error),
    NoteNotFound,
}

// `Display` determines how the error looks when printed
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "Error in accessign database: {}", e),
            Error::Io(e) => write!(f, "File operation failed: {}", e),
            Error::Reqwest(e) => write!(f, "Could not contact the server: {}", e),
            Error::Keyring(e) => write!(f, "Error in getting login token: {}", e),
            Error::NoteNotFound => write!(f, "Note not found"),
        }
    }
}


// Tells Rust that `Error` type follows the standard std::error::Error` trait.
impl std::error::Error for Error {}

// Automatically convert libsql::Error into custom Error type
impl From<libsql::Error> for Error {
    fn from(error: libsql::Error) -> Self {
        Error::Database(error)
    }
}

// Automatically convert std::io::Error into custom Error type
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

// Automatically convert reqwest::Error into custom Error type
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

// Automatically convert keyring::Error into custom Error type
impl From<keyring::Error> for Error {
    fn from(error: keyring::Error) -> Self {
        Error::Keyring(error)
    }
}

// Alias for Rust's standard Result type
pub type Result<T> = std::result::Result<T, Error>;