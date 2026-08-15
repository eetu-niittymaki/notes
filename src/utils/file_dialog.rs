use std::path::PathBuf;

use rfd::FileDialog;

use crate::config::IMPORT_FILETYPES;

pub fn folder() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("/")
        .pick_folder()
}

pub fn file() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("/")
        .set_title("Select File To Import")
        .add_filter("Supported Filetypes", &IMPORT_FILETYPES)
        .pick_file()
}