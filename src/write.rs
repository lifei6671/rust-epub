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
pub(crate) fn create_media_folder(root_dir: &Path,media_folder_name:&str,hashmap :&mut DashMap<String,String>)
    -> Result<(), Error> {
    if !hashmap.is_empty() {
        let media_folder_path = root_dir.join(epub::CONTENT_FOLDER_NAME).join(media_folder_name);
        if let Err(e) = fs::create_dir_all(&media_folder_path) {
            return Err(Error::PathCreateErr(format!("Could not create media folder:{}", e)));
        }
    }
    Ok(())
}

/// write file
pub(crate) fn write_file(output_path: &Path,content:&str) -> Result<(), Error> {
    if let Err(e) = fs::write(output_path, content) {
        return  Err(Error::PathCreateErr(format!("Could not create file:{}", e)))
    }
    Ok(())
}

/// copy media file
pub(crate) fn write_media_file (root_dir: &Path,media_folder_name:&str,hashmap :&DashMap<String,String>)
    -> Result<(), Error> {
    if !hashmap.is_empty() {
        for item in hashmap.iter() {
            let media_folder_path = root_dir.
                join(epub::CONTENT_FOLDER_NAME).
                join(media_folder_name).
                join(item.key());
            let source_path = Path::new(item.value());
            if let Err(e) = fs::copy(source_path,media_folder_path) {
                return  Err(Error::PathCreateErr(format!("Could not copy file:{}", e)))
            }
        }
    }
    Ok(())

 }
