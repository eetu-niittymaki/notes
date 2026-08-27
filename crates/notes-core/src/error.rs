use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Error from SQLite/database operation.
    Database(libsql::Error),
    // Error from input/output operations,
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Keyring(keyring::Error),
    SerdeJson(serde_json::Error),
    Argon2PasswordHash(argon2::password_hash::Error),
    JsonWebToken(jsonwebtoken::errors::Error),
    NoteNotFound,
    UserNotFound,
    UserAlreadyExists,
    Unauthorized
}

// `Display` determines how the error looks when printed
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "Error in accessing database: {}", e),
            Error::Io(e) => write!(f, "File operation failed: {}", e),
            Error::Reqwest(e) => write!(f, "Error in contacting the server: {}", e),
            Error::Keyring(e) => write!(f, "Error in getting login token: {}", e),
            Error::SerdeJson(e) => write!(f, "Error with serde_json: {}", e),
            Error::Argon2PasswordHash(e) => write!(f, "Error with Argon2 password hashing: {}", e),
            Error::JsonWebToken(e)=> write!(f, "Error with jsonwebtoken: {}", e),
            Error::NoteNotFound => write!(f, "Note not found"),
            Error::UserNotFound => write!(f, "User not found"),
            Error::UserAlreadyExists => write!(f, "Username is already taken"),
            Error::Unauthorized => write!(f, "Your session has ended. Please login again"),
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

// std::io::Error
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

// reqwest::Error
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

// keyring::Error
impl From<keyring::Error> for Error {
    fn from(error: keyring::Error) -> Self {
        Error::Keyring(error)
    }
}

// serde_json::Error 
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}

// argon2::password_hash::Error
impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        Error::Argon2PasswordHash(error)
    }
}

// jsonwebtoken::errors::Error
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::JsonWebToken(error)
    }
}

// Alias for Rust's standard Result type
pub type Result<T> = std::result::Result<T, Error>;