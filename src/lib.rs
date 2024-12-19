pub mod epub;
pub mod toc;
pub mod opf;

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

}