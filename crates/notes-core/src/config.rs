use std::path::PathBuf;

pub const LOCAL_DB_NAME: &str = "notes.db";

pub fn get_local_db_path() -> PathBuf {
    if cfg!(debug_assertions) {
        std::env::current_dir().expect("Failed to get current dir").join(LOCAL_DB_NAME)
    } else {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        exe_dir.join(LOCAL_DB_NAME)
    }
}