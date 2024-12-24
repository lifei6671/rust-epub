use dashmap::DashMap;
use quick_xml::se::Serializer;
use serde::Serialize;
use std::path::{Path, PathBuf};

pub mod epub;
pub mod mime;
pub mod opf;
pub mod toc;
mod write;
pub mod xhtml;

/// 错误信息枚举
#[derive(Debug)]
pub enum Error {
    /// Returned when the provided metadata is not a supported value for epub files.
    InvalidMetadataErr(String),
    /// Returned when an IO exception occurs in the file system.
    IOError {
        /// IO错误信息
        msg: String,
        /// IO错误的上下文信息
        cause: std::io::Error,
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

    /// Path creation error
    PathCreateErr(String),

    /// parent filename already exists
    ParentExistedErr(String),

    /// Filename already exists error
    FilenameExistedErr(String),

    /// Media file error
    MediaError(String),

    /// Serialize error
    SerializeErr(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError {
            msg: format!("{}", e),
            cause: e,
        }
    }
}

/// 将多个目录名组合成一个路径字符串
#[allow(dead_code)]
fn combine_dirs(dir_names: &[&str]) -> String {
    let mut path = PathBuf::new();
    for dir_name in dir_names {
        path = path.join(dir_name);
    }
    path.to_str().unwrap().to_string()
}

/// 添加媒体文件
fn add_media<S1: Into<String>, S2: Into<String>>(
    source: S1,
    internal_filename: Option<String>,
    media_file_format: String,
    media_folder_name: S2,
    hashmap: &DashMap<String, String>,
) -> Result<String, Error> {
    let source_str = source.into();
    // Check if file exists
    if !Path::new(&source_str).exists() {
        return Err(Error::FileNotFoundErr(format!(
            "File not found:{}",
            &source_str
        )));
    }
    let filename = internal_filename.unwrap_or_else(|| {
        let file_path = Path::new(&source_str);
        let basename = file_path.file_name().unwrap().to_str().unwrap();

        if basename.len() > 255 || hashmap.contains_key(basename) {
            let ext = file_path.extension().and_then(|osstr| osstr.to_str()).unwrap_or("jpg");
            format!("{}_{}.{}", media_file_format, hashmap.len() + 1, ext)
        } else {
            basename.to_string()
        }
    });

    if hashmap.contains_key(&filename) {
        return Err(Error::FilenameUsedErr(format!(
            "Filename already used:{}",
            &filename
        )));
    }

    hashmap.insert(filename.clone(), source_str);

    Ok(format!("../{}/{}", media_folder_name.into(), filename))
}

/// Serialize struct into a String.
pub fn encode_xml<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + Serialize,
{
    let mut buffer = String::new();
    let mut serializer = Serializer::new(&mut buffer);
    serializer.indent(' ', 2);

    value
        .serialize(serializer)
        .map_err(|e| Error::SerializeErr(format!("{}", e)))?;

    // 转换缓冲区内容为字符串
    String::from_utf8(buffer.into_bytes()).map_err(|e| Error::SerializeErr(e.to_string()))
}
