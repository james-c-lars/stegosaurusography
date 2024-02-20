// Instead of exporting all of our modules, we can selectively export the relevant parts
mod decoder;
mod encoder;
mod error;
mod file_types;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::{CorruptionType, Error, ErrorContext, ErrorType, Result, WhichDuplicates};
pub use file_types::base_file::{get_properties, FileProperties};

// TODO: We should also encode the file type in the header, so that the user doesn't
// have to guess what kind of secret file was encoded into the base file
// TODO: Should we have some sort of versioning in the header to represent what version
// of our algorithm was used?
// TODO: Should we have a dedicated header struct that handles creating itself from a
// generic bytestream?
/// The number of bytes we'll use as a header for our data.
///
/// These bytes will represent how many bytes in total are encoded.
const HEADER_BYTES: u64 = 8;
