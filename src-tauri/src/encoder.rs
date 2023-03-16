use std::{fs::File, path::PathBuf};

use crate::encoded_file::{EncodedFile, SupportedFileType};

/// Handles the steganographic process of encoding a hidden file inside of a base file.
pub struct Encoder {
    base_file: File,
    output_file: EncodedFile,
}

impl Encoder {
    /// Constructs a new Encoder.
    pub fn new(base_file: File, base_file_path: PathBuf, output_file: File) -> Option<Encoder> {
        let output_encoded_file = EncodedFile::new(output_file, base_file_path)?;

        Some(Encoder {
            base_file,
            output_file: output_encoded_file,
        })
    }

    /// Returns the number of bytes available to encode a file.
    pub fn available_space(&self) -> u64 {
        todo!()
    }

    /// Encodes the hidden file into the base file, and writes the results to the output file.
    pub fn write_hidden_file(&mut self, hidden_file: File) -> Result<(), String> {
        let metadata = hidden_file
            .metadata()
            .map_err(|_| format!("{:?}'s metadata could not be retrieved", hidden_file))?;

        let space = self.available_space();
        if metadata.len() > space {
            return Err(format!(
                "{:?} is bigger than the available encodable space of {} bytes",
                hidden_file, space
            ));
        }

        match self.output_file.file_type() {
            SupportedFileType::PNG => todo!(),
        }
    }
}
