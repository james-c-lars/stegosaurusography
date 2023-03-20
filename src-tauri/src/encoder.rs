use std::{
    fs::{canonicalize, File},
    path::PathBuf,
};

use crate::{
    error::{DuplicateEnum, Error},
    file_types::base_file::BaseFile,
};

/// Handles the steganographic process of encoding a hidden file inside of a base file.
pub struct Encoder {
    base_file: BaseFile,
    secret_file: File,
    output_file: File,
}

impl Encoder {
    /// Constructs a new Encoder.
    pub fn new(
        base_file_path: PathBuf,
        secret_file_path: PathBuf,
        output_file_path: PathBuf,
    ) -> Result<Encoder, Error> {
        let base_file = BaseFile::open(base_file_path.clone())?;
        let secret_file = File::open(secret_file_path.clone())?;
        let output_file = File::create(output_file_path.clone())?;

        Encoder::check_for_duplicate_files(base_file_path, secret_file_path, output_file_path)?;

        Ok(Encoder {
            base_file,
            secret_file,
            output_file,
        })
    }

    /// Checks to see if any of the given files are the same.
    fn check_for_duplicate_files(
        base_file_path: PathBuf,
        secret_file_path: PathBuf,
        output_file_path: PathBuf,
    ) -> Result<(), Error> {
        let canonicalized_base = canonicalize(base_file_path)?;
        let canonicalized_secret = canonicalize(secret_file_path)?;
        let canonicalized_output = canonicalize(output_file_path)?;

        if canonicalized_base == canonicalized_secret {
            if canonicalized_secret == canonicalized_output {
                Err(Error::DuplicateFiles(DuplicateEnum::All))
            } else {
                Err(Error::DuplicateFiles(DuplicateEnum::BaseAndSecret))
            }
        } else if canonicalized_base == canonicalized_output {
            Err(Error::DuplicateFiles(DuplicateEnum::BaseAndOutput))
        } else if canonicalized_secret == canonicalized_output {
            Err(Error::DuplicateFiles(DuplicateEnum::SecretAndOutput))
        } else {
            Ok(())
        }
    }

    /// Encodes the hidden file into the base file, and writes the results to the output file.
    pub fn encode(&mut self) -> Result<(), Error> {
        self.base_file
            .encode_to(&self.secret_file, &mut self.output_file)
    }
}
