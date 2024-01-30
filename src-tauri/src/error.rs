use std::{io::Error as IOError, path::Path};

use image::ImageError;
use serde::Serialize;

/// A shorthand for Result<T, crate::Error>.
///
/// Examples of the serialization of this type to JSON:
/// {
///     "Err": {
///         "contexts": ["BaseFile"],
///         "type": "IOError",
///         "content": "The system cannot find the file specified. (os error 2)"
///     }
/// }
///
/// {
///     "Ok": {
///         "available_space": 3462648,
///         "file_type": "Png"
///     }
/// }
pub type Result<T> = std::result::Result<T, Error>;

/// Contains information about an error that occurred while processing a request.
#[derive(Debug, Serialize)]
pub struct Error {
    /// Which files the error is associated with. Most often a single file.
    pub contexts: Vec<ErrorContext>,
    /// What the type of error was that occurred.
    #[serde(flatten)]
    pub error_type: ErrorType,
}

/// A representation of which file in the steganography process caused the error.
#[derive(Debug, Serialize)]
pub enum ErrorContext {
    /// The file into which data is encoded.
    BaseFile,
    /// The file that is hidden inside a base file.
    SecretFile,
    /// A file with a secret hidden in it.
    EncodedFile,
    /// A file used to output the results of an operation.
    OutputFile,
}

/// Used to both add a context to an ErrorType, but also log the error inline using the log crate.
macro_rules! with_contexts {
    ($result:expr, $($context:expr),+) => {
        $result.map_err(|err| {
            let error_type = crate::error::ErrorType::from(err);
            let contexts = vec![$($context,)+];

            log::error!("Context {contexts:?} - {error_type:?}");

            crate::Error {
                contexts,
                error_type,
            }
        })
    };
}
pub(crate) use with_contexts;

/// Gives a BaseFile context to an error.
macro_rules! base_context {
    ($result:expr) => {
        crate::error::with_contexts!($result, crate::error::ErrorContext::BaseFile)
    };
}
pub(crate) use base_context;

/// Gives a SecretFile context to an error.
macro_rules! secret_context {
    ($result:expr) => {
        crate::error::with_contexts!($result, crate::error::ErrorContext::SecretFile)
    };
}
pub(crate) use secret_context;

/// Gives an OutputFile context to an error.
macro_rules! output_context {
    ($result:expr) => {
        crate::error::with_contexts!($result, crate::error::ErrorContext::OutputFile)
    };
}
pub(crate) use output_context;

/// Gives an EncodedFile context to an error.
macro_rules! encoded_context {
    ($result:expr) => {
        crate::error::with_contexts!($result, crate::error::ErrorContext::EncodedFile)
    };
}
pub(crate) use encoded_context;

/// Different types of errors associated with Encoding and Decoding files.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum ErrorType {
    /// An error due to reading or writing from the file system.
    IOError(String),
    /// An error due to trying to perform an operation on an unsupported file type.
    UnsupportedFileType(Box<Path>),
    /// An error due to trying to encode a secret file into a base file too small to contain it.
    BaseFileNotBigEnough {
        /// The amount of space available in the base file for encoding.
        available_size: u64,
        /// The amount of space the secret file requires to be encoded into.
        secret_file_size: u64,
    },
    /// An error due to trying to perform an operation with the same file serving multiple roles.
    DuplicateFiles(WhichDuplicates),
    /// An error due to an image file in an uninterpretable format.
    ImageError(String),
    /// An error due to an encoded file in an uninterpretable format.
    CorruptedFile(CorruptionType),
}

impl From<IOError> for ErrorType {
    fn from(value: IOError) -> Self {
        ErrorType::IOError(value.to_string())
    }
}

impl From<ImageError> for ErrorType {
    fn from(value: ImageError) -> Self {
        if let ImageError::IoError(io_error) = value {
            return io_error.into();
        }

        ErrorType::ImageError(value.to_string())
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

/// Represents how an encoded file was corrupted.
#[derive(Debug, Serialize)]
pub enum CorruptionType {
    /// The header to the secret data was in an uninterpretable format.
    IncorrectHeader,
    /// The encoded file is too small to contain data.
    FileTooSmallForHeader,
}
