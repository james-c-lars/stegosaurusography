use std::fs::File;

use crate::encoded_file::{EncodedFile, SupportedFileType};

/// Handles the steganographic process of decoding an encoded file.
pub struct Decoder {
    input_file: EncodedFile,
    output_file: File,
}

impl Decoder {
    /// Constructs a new Decoder.
    pub fn new(input_file: File, input_file_name: String, output_file: File) -> Option<Decoder> {
        let encoded_file = EncodedFile::new(input_file, input_file_name)?;

        Some(Decoder {
            input_file: encoded_file,
            output_file,
        })
    }

    /// Decodes the input file, and writes the results to the output file.
    pub fn read_encoded_file(&mut self) -> Result<(), String> {
        match self.input_file.file_type() {
            SupportedFileType::PNG => todo!(),
            SupportedFileType::TXT => todo!(),
        }
    }
}
