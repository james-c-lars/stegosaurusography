use std::{io::Error as IOError, path::PathBuf};

use serde::Serialize;

/// Different errors associated with Encoding and Decoding files.
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Error {
    IOError(String),
    UnsupportedFileType(PathBuf),
    BaseFileNotBigEnough {
        available_size: u64,
        secret_file_size: u64,
    },
    DuplicateFiles(DuplicateEnum),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Error::IOError(value.to_string())
    }
}

/// Represents different sets of files that may be duplicates when encoding.
#[derive(Serialize)]
pub enum DuplicateEnum {
    BaseAndSecret,
    SecretAndOutput,
    BaseAndOutput,
    All,
}
