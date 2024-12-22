use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod epub;
pub mod toc;
pub mod opf;
pub mod mime;
pub mod xhtml;
mod raw_xml;

/// 错误信息枚举
#[derive(Debug)]
pub enum Error {
    /// Returned when the provided metadata is not a supported value for epub files.
    InvalidMetadataErr(String),
    /// Returned when an IO exception occurs in the file system.
    IOError{
        /// IO错误信息
        msg :String,
        /// IO错误的上下文信息
        cause : std::io::Error,
    },
    /// This error may be that the path is unreadable, does not exist, or has abnormal path permissions.
    PathError(String),
    /// Returns when the supplied filename is duplicated.
    FilenameUsedErr(String),

    /// Encoding error
    NonEncodable(String),

    /// Mime type error
    MimeError(String),

    /// Path conversion error
    PathConversionErr(String),
    /// File not found error
    FileNotFoundErr(String),

}
fn combine_dirs(dir_names: &[&str]) -> String {
    let mut path = PathBuf::new();
    for dir_name in dir_names {
        path = path.join(dir_name);
    }
    path.to_str().unwrap().to_string()
}
fn add_media<S1: Into<String>, S2: Into<String>>(
    source: S1,
    internal_filename: Option<String>,
    media_file_format: String,
    media_folder_name: S2,
    hashmap: &mut HashMap<String, String>,
) -> Result<String, Error> {
    let source_str = source.into();
    /// Check if file exists
    if !Path::new(&source_str).exists() {
        return Err(Error::FileNotFoundErr(format!(
            "File not found:{}",
            &source_str
        )));
    }
    // 判断是否指定了内部文件名
    let filename = if let Some(internal_filename) = internal_filename {
        internal_filename
    } else {
        let file_path = Path::new(&source_str);
        let basename = file_path.file_name().unwrap().to_str().unwrap();
        /// 判断文件名是否过长或是否已被使用
        if basename.len() > 255 || hashmap.contains_key(basename) {
            let ext = file_path
                .extension()
                .and_then(|osstr| osstr.to_str())
                .unwrap_or("jpg");
            format!("{}_{}.{}", media_file_format, hashmap.len() + 1, ext)
        } else {
            String::from(basename)
        }
    };

    if hashmap.contains_key(&filename) {
        return Err(Error::FilenameUsedErr(format!(
            "Filename already used:{}",
            &filename
        )));
    }
    hashmap.insert(filename.clone(), source_str);
    Ok(format!("../{}/{}", media_folder_name.into(), filename))
}
