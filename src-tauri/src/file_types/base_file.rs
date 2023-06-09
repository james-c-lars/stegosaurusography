use std::{fs::File, path::PathBuf};

use serde::Serialize;

use crate::{
    error::Error,
    file_types::{
        png,
        supported_file::{SupportedFile, SupportedFileType},
    },
};

/// A collection of properties of the base file.
#[derive(Serialize)]
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
    pub fn open(file_path: PathBuf) -> Result<BaseFile, Error> {
        Ok(BaseFile {
            file: SupportedFile::open(file_path)?,
        })
    }

    /// Returns the number of bytes available to encode a file.
    pub fn available_space(&self) -> Result<u64, Error> {
        match self.file.file_type() {
            SupportedFileType::Png => png::available_size_of(&self.file),
        }
    }

    /// Gets the properties of the BaseFile.
    pub fn get_properties(&self) -> Result<FileProperties, Error> {
        Ok(FileProperties {
            available_space: self.available_space()?,
            file_type: self.file.file_type(),
        })
    }

    /// Encodes the secret file into this base file and outputs the results.
    pub fn encode_to(&self, secret_file: &File, output_file: &mut File) -> Result<(), Error> {
        let available_size = self.available_space()?;
        let secret_file_size = secret_file.metadata()?.len();

        if secret_file_size > available_size {
            return Err(Error::BaseFileNotBigEnough {
                available_size,
                secret_file_size,
            });
        }

        match self.file.file_type() {
            SupportedFileType::Png => png::encode(&self.file, secret_file, output_file),
        }
    }
}

/// Gets the properties of a file including its type and how much space is available.
pub fn get_properties(base_file_path: PathBuf) -> Result<FileProperties, Error> {
    BaseFile::open(base_file_path)?.get_properties()
}
