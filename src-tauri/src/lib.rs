// Instead of exporting all of our modules, we can selectively export the relevant parts
mod decoder;
mod encoder;
mod error;
mod file_types;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::Error;
pub use file_types::base_file::{get_properties, FileProperties};
