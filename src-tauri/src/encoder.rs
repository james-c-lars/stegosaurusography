use std::{fs::File, path::PathBuf};

use crate::{error::Error, file_types::base_file::BaseFile};

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
        let base_file = BaseFile::open(base_file_path)?;
        let secret_file = File::open(secret_file_path)?;
        let output_file = File::create(output_file_path)?;

        Ok(Encoder {
            base_file,
            secret_file,
            output_file,
        })
    }

    /// Encodes the hidden file into the base file, and writes the results to the output file.
    pub fn encode(&mut self) -> Result<(), Error> {
        self.base_file
            .encode_to(&self.secret_file, &mut self.output_file)
    }
}
