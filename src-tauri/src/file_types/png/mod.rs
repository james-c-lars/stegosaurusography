use std::fs::File;

use crate::error::Error;

/// Finds the amount of space in bytes, that can be used to store a secret file.
pub fn available_size_of(file: &File) -> Result<u64, Error> {
    todo!()
}

/// Encodes the hidden file into the base PNG, and writes the results to the output PNG.
pub fn encode(base_png: &File, secret_file: &File, output_png: &mut File) -> Result<(), Error> {
    todo!()
}

/// Decodes the encoded PNG, and writes the results to the output file.
pub fn decode(encoded_png: &File, output_file: &mut File) -> Result<(), Error> {
    todo!()
}
