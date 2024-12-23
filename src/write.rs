use crate::{epub, Error};
use dashmap::DashMap;
use std::fs;
use std::path::Path;

/// Create the folders for an epub
pub(crate) fn create_epub_folders(root_dir: &Path) -> Result<(), Error> {
    // Create the root folder
    if let Err(e) = fs::create_dir_all(root_dir.join(epub::CONTENT_FOLDER_NAME)) {
        return Err(Error::PathCreateErr(format!("Could not create content folder:{}", e)));
    }
    // Create the meta-inf folder
    if let Err(e) = fs::create_dir_all(root_dir.join(epub::META_INF_FOLDER_NAME)) {
        return Err(Error::PathCreateErr(format!("Could not create meta-inf folder:{}", e)));
    }
    Ok(())
}

/// Create media folder
pub(crate) fn create_media_folder(root_dir: &Path,media_folder_name:&str,hashmap :&mut DashMap<String,String>) -> Result<(), Error> {
    if !hashmap.is_empty() {
        let media_folder_path = root_dir.join(epub::CONTENT_FOLDER_NAME).join(media_folder_name);
        if let Err(e) = fs::create_dir_all(&media_folder_path) {
            return Err(Error::PathCreateErr(format!("Could not create media folder:{}", e)));
        }
    }
    Ok(())
}
