// Instead of exporting all of our modules, we can selectively export the relevant parts
mod decoder;
mod encoder;
mod error;
mod file_types;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::{CorruptionType, Error, WhichDuplicates};
pub use file_types::base_file::{get_properties, FileProperties};

/// The number of bytes we'll use as a header for our data.
///
/// These bytes will represent how many bytes in total are encoded.
const HEADER_BYTES: u64 = 8;

pub use error::Result;
