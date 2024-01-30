use std::{
    fs::{canonicalize, File},
    path::Path,
};

use crate::{
    encoded_context, file_types::encoded_file::EncodedFile, output_context, with_contexts,
    ErrorContext, ErrorType, Result, WhichDuplicates,
};

/// Handles the steganographic process of decoding an encoded file.
pub struct Decoder {
    encoded_file: EncodedFile,
    output_file: File,
}

impl Decoder {
    /// Constructs a new Decoder.
    pub fn new(
        encoded_file_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<Decoder> {
        Decoder::check_for_duplicate_files(&encoded_file_path, &output_file_path)?;
        log::trace!("Ensured no duplicate files");

        let encoded_file = EncodedFile::open(&encoded_file_path)?;
        let output_file = output_context!(File::create(&output_file_path))?;
        log::trace!("Opened all files");

        Ok(Decoder {
            encoded_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        encoded_file_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<()> {
        let canonicalized_encoded = encoded_context!(canonicalize(encoded_file_path))?;
        let canonicalized_output = output_context!(canonicalize(output_file_path))?;
        log::trace!("Canonicalized file paths");

        if canonicalized_encoded == canonicalized_output {
            with_contexts!(
                Err(ErrorType::DuplicateFiles(WhichDuplicates::All)),
                ErrorContext::EncodedFile,
                ErrorContext::OutputFile
            )
        } else {
            Ok(())
        }
    }

    /// Decodes the encoded file, and writes the results to the output file.
    pub fn decode(&mut self) -> Result<()> {
        self.encoded_file.decode_to(&mut self.output_file)
    }
}
