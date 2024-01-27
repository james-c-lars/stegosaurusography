use std::{fs::File, path::Path};

use crate::{
    error::Error,
    file_types::{
        image,
        supported_file::{SupportedFile, SupportedFileType},
    },
};

/// An encoded file is a file with another secret file encoded into it.
pub struct EncodedFile {
    file: SupportedFile,
}

impl EncodedFile {
    /// Opens a file that has another secret file encoded in it.
    pub fn open(file_path: &Path) -> Result<EncodedFile, Error> {
        Ok(EncodedFile {
            file: SupportedFile::open(file_path)?,
        })
    }

    /// Decodes the secret file inside this one to the output file.
    pub fn decode_to(&self, output_file: &mut File) -> Result<(), Error> {
        log::trace!("Beginning the decoding process");
        match self.file.file_type() {
            SupportedFileType::Png => image::decode(&self.file, output_file),
        }
    }
}
