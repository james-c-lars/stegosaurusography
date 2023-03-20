use std::{
    fs::{canonicalize, File},
    path::PathBuf,
};

use crate::{
    error::{DuplicateEnum, Error},
    file_types::encoded_file::EncodedFile,
};

/// Handles the steganographic process of decoding an encoded file.
pub struct Decoder {
    encoded_file: EncodedFile,
    output_file: File,
}

impl Decoder {
    /// Constructs a new Decoder.
    pub fn new(encoded_file_path: PathBuf, output_file_path: PathBuf) -> Result<Decoder, Error> {
        let encoded_file = EncodedFile::open(encoded_file_path.clone())?;
        let output_file = File::create(output_file_path.clone())?;

        Decoder::check_for_duplicate_files(encoded_file_path, output_file_path)?;

        Ok(Decoder {
            encoded_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        encoded_file_path: PathBuf,
        output_file_path: PathBuf,
    ) -> Result<(), Error> {
        let canonicalized_encoded = canonicalize(encoded_file_path)?;
        let canonicalized_output = canonicalize(output_file_path)?;

        if canonicalized_encoded == canonicalized_output {
            Err(Error::DuplicateFiles(DuplicateEnum::All))
        } else {
            Ok(())
        }
    }

    /// Decodes the encoded file, and writes the results to the output file.
    pub fn decode(&mut self) -> Result<(), Error> {
        self.encoded_file.decode_to(&mut self.output_file)
    }
}
