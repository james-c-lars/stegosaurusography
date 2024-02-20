use std::path::Path;
use std::{
    fs::File,
    ops::{Deref, DerefMut},
};

use image::ImageFormat;
use serde::Serialize;

use crate::error::ErrorType;

/// Represents a type of file that we support encoding a secret file into.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum SupportedFileType {
    Png,
    // TODO: Test which image types we can directly plug in with our existing image module
    // Any sort of non-lossy format should probably work out-of-the-box with what we already have
    // TODO: Add support for Open Office format files
    // TODO: Add support for PDFs
    // TODO: Add support for audio files
    // TODO: Add support for video / GIFs
}

impl SupportedFileType {
    /// Finds the file type from a file's name.
    ///
    /// Returns None if the file type is not supported.
    pub fn from_file_path<P: AsRef<Path>>(file_path: P) -> Option<SupportedFileType> {
        let file_path = file_path.as_ref();
        let extension = file_path.extension()?;
        log::trace!("Parsed file extension");

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
    pub fn open<P: AsRef<Path>>(file_path: P) -> Result<SupportedFile, ErrorType> {
        SupportedFileType::from_file_path(&file_path)
            .ok_or(ErrorType::UnsupportedFileType(file_path.as_ref().into()))
            .and_then(|file_type| {
                let file = File::open(&file_path)?;
                Ok(SupportedFile { file, file_type })
            })
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
