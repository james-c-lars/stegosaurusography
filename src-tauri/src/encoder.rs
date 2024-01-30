use std::{
    fs::{canonicalize, File},
    path::Path,
};

use crate::{
    base_context, file_types::base_file::BaseFile, output_context, secret_context, with_contexts,
    ErrorType, Result, WhichDuplicates,
};

/// Handles the steganographic process of encoding a hidden file inside a base file.
pub struct Encoder {
    base_file: BaseFile,
    secret_file: File,
    output_file: File,
}

impl Encoder {
    /// Constructs a new Encoder.
    pub fn new(
        base_file_path: impl AsRef<Path>,
        secret_file_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<Encoder> {
        Encoder::check_for_duplicate_files(&base_file_path, &secret_file_path, &output_file_path)?;
        log::trace!("Ensured no duplicate files");

        let base_file = BaseFile::open(&base_file_path)?;
        let secret_file = secret_context!(File::open(&secret_file_path))?;
        let output_file = output_context!(File::create(&output_file_path))?;
        log::trace!("Opened all files");

        Ok(Encoder {
            base_file,
            secret_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        base_file_path: impl AsRef<Path>,
        secret_file_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<()> {
        let canonicalized_base = base_context!(canonicalize(base_file_path))?;
        let canonicalized_secret = secret_context!(canonicalize(secret_file_path))?;
        let canonicalized_output = output_context!(canonicalize(output_file_path))?;
        log::trace!("Canonicalized file paths");

        use crate::error::ErrorContext::*;
        if canonicalized_base == canonicalized_secret {
            if canonicalized_secret == canonicalized_output {
                with_contexts!(
                    Err(ErrorType::DuplicateFiles(WhichDuplicates::All)),
                    BaseFile,
                    SecretFile,
                    OutputFile
                )
            } else {
                with_contexts!(
                    Err(ErrorType::DuplicateFiles(WhichDuplicates::BaseAndSecret)),
                    BaseFile,
                    SecretFile
                )
            }
        } else if canonicalized_base == canonicalized_output {
            with_contexts!(
                Err(ErrorType::DuplicateFiles(WhichDuplicates::BaseAndOutput)),
                BaseFile,
                OutputFile
            )
        } else if canonicalized_secret == canonicalized_output {
            with_contexts!(
                Err(ErrorType::DuplicateFiles(WhichDuplicates::SecretAndOutput)),
                SecretFile,
                OutputFile
            )
        } else {
            Ok(())
        }
    }

    /// Encodes the hidden file into the base file, and writes the results to the output file.
    pub fn encode(&mut self) -> Result<()> {
        self.base_file
            .encode_to(&self.secret_file, &mut self.output_file)
    }
}
