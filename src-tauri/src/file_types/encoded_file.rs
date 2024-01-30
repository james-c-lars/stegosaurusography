use std::{fs::File, path::Path};

use crate::{
    encoded_context,
    file_types::{
        image,
        supported_file::{SupportedFile, SupportedFileType},
    },
    Result,
};

/// An encoded file is a file with another secret file encoded into it.
pub struct EncodedFile {
    file: SupportedFile,
}

impl EncodedFile {
    /// Opens a file that has another secret file encoded in it.
    pub fn open(file_path: impl AsRef<Path>) -> Result<EncodedFile> {
        encoded_context!(SupportedFile::open(file_path)).map(|file| EncodedFile { file })
    }

    /// Decodes the secret file inside this one to the output file.
    pub fn decode_to(&self, output_file: &mut File) -> Result<()> {
        match self.file.file_type() {
            SupportedFileType::Png => image::decode(&self.file, output_file),
        }
    }
}
