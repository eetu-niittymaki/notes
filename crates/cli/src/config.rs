use std::path::PathBuf;

pub const DB_NAME: &str = "notes.db";

pub const URL: &str  = "http://localhost:8080/";

pub const IMPORT_FILETYPES: [&str; 3] = ["md", "txt", "html"];

pub const EXPORT_FILETYPES: [&str; 6] = ["md", "txt", "html", "json", "png", "pdf"];

pub fn get_db_path() -> PathBuf {
    if cfg!(debug_assertions) {
        std::env::current_dir().expect("Failed to get current dir").join(DB_NAME)
    } else {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        exe_dir.join(DB_NAME)
    }
}