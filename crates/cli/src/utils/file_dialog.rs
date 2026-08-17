use std::path::PathBuf;

use rfd::FileDialog;

use crate::config::IMPORT_FILETYPES;

pub fn folder(title: &str) -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("/")
        .set_title(title)
        .pick_folder()
}

pub fn file(title: &str) -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("/")
        .set_title(title)
        .add_filter("Supported Filetypes", &IMPORT_FILETYPES)
        .pick_file()
}