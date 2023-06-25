use std::{
    fs::File,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::Serialize;

use crate::error::Error;

/// Represents a type of file that we support encoding a secret file into.
#[derive(Clone, Copy, Serialize)]
pub enum SupportedFileType {
    Png,
}

impl SupportedFileType {
    /// Finds the file type from a file's name.
    ///
    /// Returns None if the file type is not supported.
    pub fn from_file_path(file_path: PathBuf) -> Option<SupportedFileType> {
        match file_path.extension()?.to_ascii_lowercase().to_str()? {
            "png" => Some(SupportedFileType::Png),
            _ => None,
        }
    }
}

/// A wrapper for File that includes what type of file it is.
pub struct SupportedFile {
    file: File,
    file_type: SupportedFileType,
}

impl SupportedFile {
    /// Attempts to open an existing file.
    ///
    /// Returns an error if the file isn't a supported type or if the file can't be opened.
    pub fn open(file_path: PathBuf) -> Result<SupportedFile, Error> {
        let file = File::open(file_path.clone())?;

        if let Some(file_type) = SupportedFileType::from_file_path(file_path.clone()) {
            Ok(SupportedFile { file, file_type })
        } else {
            Err(Error::UnsupportedFileType(file_path))
        }
    }

    /// Returns the file type as an enum.
    pub fn file_type(&self) -> SupportedFileType {
        self.file_type
    }
}

/// Implementing Deref and DerefMut is what allows us to use File methods directly from
/// SupportedFile without any need for explicit conversion.

impl Deref for SupportedFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for SupportedFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}
