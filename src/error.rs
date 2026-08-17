use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Error from SQLite/database operation.
    Database(rusqlite::Error),
    // Error from input/output operations,
    Io(std::io::Error),
}

// `Display` determines how the error looks when printed
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "database error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}


// Tells Rust that `Error` type follows the standard std::error::Error` trait.
impl std::error::Error for Error {}

// Automatically convert rusqlite::Error into custom Error type
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Database(error)
    }
}

// Automatically convert std::io::Error into custom Error type
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

// Alias for Rust's standard Result type
pub type Result<T> = std::result::Result<T, Error>;