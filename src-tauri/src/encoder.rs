use std::{
    fs::{canonicalize, File},
    path::{Path, PathBuf},
};

use crate::{
    error::{Error, WhichDuplicates},
    file_types::base_file::BaseFile,
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
        base_file_path: impl Into<PathBuf>,
        secret_file_path: impl Into<PathBuf>,
        output_file_path: impl Into<PathBuf>,
    ) -> Result<Encoder, Error> {
        let base_file_path = base_file_path.into();
        let secret_file_path = secret_file_path.into();
        let output_file_path = output_file_path.into();

        log::trace!("Opening files");
        let base_file = BaseFile::open(&base_file_path)?;
        let secret_file = File::open(&secret_file_path)?;
        let output_file = File::create(&output_file_path)?;

        log::trace!("Checking for duplicates");
        Encoder::check_for_duplicate_files(&base_file_path, &secret_file_path, &output_file_path)?;

        Ok(Encoder {
            base_file,
            secret_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        base_file_path: &Path,
        secret_file_path: &Path,
        output_file_path: &Path,
    ) -> crate::Result<()> {
        let canonicalized_base = canonicalize(base_file_path)?;
        let canonicalized_secret = canonicalize(secret_file_path)?;
        let canonicalized_output = canonicalize(output_file_path)?;

        if canonicalized_base == canonicalized_secret {
            if canonicalized_secret == canonicalized_output {
                Err(Error::DuplicateFiles(WhichDuplicates::All))
            } else {
                Err(Error::DuplicateFiles(WhichDuplicates::BaseAndSecret))
            }
        } else if canonicalized_base == canonicalized_output {
            Err(Error::DuplicateFiles(WhichDuplicates::BaseAndOutput))
        } else if canonicalized_secret == canonicalized_output {
            Err(Error::DuplicateFiles(WhichDuplicates::SecretAndOutput))
        } else {
            Ok(())
        }
    }

    /// Encodes the hidden file into the base file, and writes the results to the output file.
    pub fn encode(&mut self) -> crate::Result<()> {
        self.base_file
            .encode_to(&self.secret_file, &mut self.output_file)
    }
}
