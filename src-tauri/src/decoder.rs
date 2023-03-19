use std::{fs::File, path::PathBuf};

use crate::{error::Error, file_types::encoded_file::EncodedFile};

/// Handles the steganographic process of decoding an encoded file.
pub struct Decoder {
    encoded_file: EncodedFile,
    output_file: File,
}

impl Decoder {
    /// Constructs a new Decoder.
    pub fn new(encoded_file_path: PathBuf, output_file_path: PathBuf) -> Result<Decoder, Error> {
        let encoded_file = EncodedFile::open(encoded_file_path)?;
        let output_file = File::create(output_file_path)?;

        Ok(Decoder {
            encoded_file,
            output_file,
        })
    }

    /// Decodes the encoded file, and writes the results to the output file.
    pub fn decode(&mut self) -> Result<(), Error> {
        self.encoded_file.decode_to(&mut self.output_file)
    }
}
