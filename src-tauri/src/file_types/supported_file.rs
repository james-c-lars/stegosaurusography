use std::{
    fs::File,
    ops::{Deref, DerefMut},
    path::Path,
};

use image::ImageFormat;
use serde::Serialize;

use crate::error::Error;

/// Represents a type of file that we support encoding a secret file into.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum SupportedFileType {
    Png,
}

impl SupportedFileType {
    /// Finds the file type from a file's name.
    ///
    /// Returns None if the file type is not supported.
    pub fn from_file_path(file_path: &Path) -> Option<SupportedFileType> {
        let extension = file_path.extension()?;

        let maybe_file_path =
            ImageFormat::from_extension(extension).and_then(|image_format| match image_format {
                ImageFormat::Png => {
                    log::info!("Parsed {file_path:?} as Image (PNG)");
                    Some(SupportedFileType::Png)
                }
                _ => None,
            });

        if maybe_file_path.is_none() {
            log::debug!("Didn't match against file extension: {extension:?}");
        }

        maybe_file_path
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
    pub fn open(file_path: &Path) -> Result<SupportedFile, Error> {
        let file = File::open(file_path)?;

        match SupportedFileType::from_file_path(file_path) {
            Some(file_type) => Ok(SupportedFile { file, file_type }),
            None => Err(Error::UnsupportedFileType(file_path.into())),
        }
    }

    /// Returns the file type as an enum.
    pub fn file_type(&self) -> SupportedFileType {
        self.file_type
    }

    /// Returns the ImageFormat associated with this file if it is an image. Otherwise None.
    pub fn image_type(&self) -> Option<ImageFormat> {
        match self.file_type {
            SupportedFileType::Png => Some(ImageFormat::Png),
        }
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
