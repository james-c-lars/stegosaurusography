use std::{fs::File, path::Path};

use serde::Serialize;

use crate::{
    error::{base_context, secret_context},
    file_types::{
        image,
        supported_file::{SupportedFile, SupportedFileType},
    },
    ErrorType, Result,
};

/// A collection of properties about the base file.
#[derive(Debug, Serialize)]
pub struct FileProperties {
    pub available_space: u64,
    pub file_type: SupportedFileType,
}

/// A base file is a file that will have another secret file encoded into it.
pub struct BaseFile {
    file: SupportedFile,
}

impl BaseFile {
    /// Opens a file that will be used as the base of an encoded file.
    pub fn open(file_path: impl AsRef<Path>) -> Result<BaseFile> {
        base_context!(SupportedFile::open(file_path)).map(|file| BaseFile { file })
    }

    /// Returns the number of bytes available to encode a file.
    pub fn available_space(&self) -> Result<u64> {
        match self.file.file_type() {
            SupportedFileType::Png => base_context!(image::available_size_of(&self.file)),
        }
    }

    /// Gets the properties of the BaseFile.
    pub fn get_properties(&self) -> Result<FileProperties> {
        Ok(FileProperties {
            available_space: self.available_space()?,
            file_type: self.file.file_type(),
        })
    }

    /// Encodes the secret file into this base file and outputs the results.
    pub fn encode_to(&self, secret_file: &File, output_file: &mut File) -> Result<()> {
        let available_size = self.available_space()?;
        let secret_file_size = secret_context!(secret_file.metadata())?.len();

        if secret_file_size > available_size {
            log::debug!("Cancelling encoding due to lack of space in base file. {secret_file_size}/{available_size}");
            return base_context!(Err(ErrorType::BaseFileNotBigEnough {
                available_size,
                secret_file_size,
            }));
        }

        log::trace!("Verified that the base file is large enough to hide the secret file");

        match self.file.file_type() {
            SupportedFileType::Png => image::encode(&self.file, secret_file, output_file),
        }
    }
}

/// Gets the properties of a file including its type and how much space is available.
pub fn get_properties(base_file_path: impl AsRef<Path>) -> Result<FileProperties> {
    BaseFile::open(base_file_path)?.get_properties()
}
