use std::{
    fs::{canonicalize, File},
    path::{Path, PathBuf},
};

use crate::{
    error::{Error, WhichDuplicates},
    file_types::encoded_file::EncodedFile,
};

/// Handles the steganographic process of decoding an encoded file.
pub struct Decoder {
    encoded_file: EncodedFile,
    output_file: File,
}

impl Decoder {
    /// Constructs a new Decoder.
    pub fn new(
        encoded_file_path: impl Into<PathBuf>,
        output_file_path: impl Into<PathBuf>,
    ) -> Result<Decoder, Error> {
        let encoded_file_path = encoded_file_path.into();
        let output_file_path = output_file_path.into();

        log::trace!("Opening files");
        let encoded_file = EncodedFile::open(&encoded_file_path)?;
        let output_file = File::create(&output_file_path)?;

        log::trace!("Checking for duplicates");
        Decoder::check_for_duplicate_files(&encoded_file_path, &output_file_path)?;

        Ok(Decoder {
            encoded_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        encoded_file_path: &Path,
        output_file_path: &Path,
    ) -> crate::Result<()> {
        let canonicalized_encoded = canonicalize(encoded_file_path)?;
        let canonicalized_output = canonicalize(output_file_path)?;

        if canonicalized_encoded == canonicalized_output {
            Err(Error::DuplicateFiles(WhichDuplicates::All))
        } else {
            Ok(())
        }
    }

    /// Decodes the encoded file, and writes the results to the output file.
    pub fn decode(&mut self) -> crate::Result<()> {
        self.encoded_file.decode_to(&mut self.output_file)
    }
}
