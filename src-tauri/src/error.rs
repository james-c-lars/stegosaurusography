use std::{io::Error as IOError, path::Path};

use image::ImageError;
use serde::Serialize;

/// Different errors associated with Encoding and Decoding files.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    IOError(String),
    UnsupportedFileType(Box<Path>),
    BaseFileNotBigEnough {
        available_size: u64,
        secret_file_size: u64,
    },
    DuplicateFiles(WhichDuplicates),
    ImageError(String),
    CorruptedFile(CorruptionType),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Error::IOError(value.to_string())
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        if let ImageError::IoError(io_error) = value {
            return io_error.into();
        }

        Error::ImageError(value.to_string())
    }
}

/// Represents different sets of files that may be duplicates when encoding.
#[derive(Debug, Serialize)]
pub enum WhichDuplicates {
    BaseAndSecret,
    SecretAndOutput,
    BaseAndOutput,
    All,
}

/// Represents how an input file was corrupted when decoding.
#[derive(Debug, Serialize)]
pub enum CorruptionType {
    IncorrectHeader,
    FileTooSmallForHeader,
}
