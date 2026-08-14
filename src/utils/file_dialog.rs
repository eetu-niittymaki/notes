use std::path::PathBuf;

use rfd::FileDialog;

pub fn folder() -> PathBuf {
    let folder = FileDialog::new()
        .set_directory("/")
        .pick_folder();

    folder.unwrap()
}