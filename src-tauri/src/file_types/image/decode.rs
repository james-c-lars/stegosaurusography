use std::{
    fs::File,
    io::{BufWriter, Write},
};

use image::{DynamicImage, GenericImageView, Pixel};
use itertools::Itertools;

use crate::{
    encoded_context,
    file_types::{
        image::{coord_iter, reader_from_supported_file, TWO_BIT_MASK},
        supported_file::SupportedFile,
    },
    output_context, CorruptionType, ErrorType, Result, HEADER_BYTES,
};

/// Decodes the encoded image, and writes the results to the output file.
pub fn decode(encoded_image: &SupportedFile, output_file: &mut File) -> Result<()> {
    log::info!("Beginning the decoding process from an image");

    // Getting the image that contains the secret
    let reader = reader_from_supported_file(encoded_image);
    let image = encoded_context!(reader.decode())?;
    log::trace!("Parsed the encoded image");

    // Starting by reading the header to find the original file size
    let mut secret_data = byte_iterator(&image);
    let mut header_bytes = [0; HEADER_BYTES as usize];
    for header_byte in header_bytes.iter_mut() {
        match secret_data.next() {
            Some(byte) => *header_byte = byte,
            None => {
                return encoded_context!(Err(ErrorType::CorruptedFile(
                    CorruptionType::FileTooSmallForHeader
                )))
            }
        }
    }
    let file_size = u64::from_be_bytes(header_bytes) as usize;

    log::trace!("Decoded the header. File size of {file_size}");

    // Writing out the secret data to the output file
    let mut writer = BufWriter::new(output_file);
    let mut bytes_written = 0;
    for byte in secret_data.take(file_size) {
        bytes_written += output_context!(writer.write(&[byte]))?;
    }

    log::trace!("Wrote encoded data to the output file");

    // Verifying that we were able to read the entire file according to the header
    if bytes_written < file_size {
        encoded_context!(Err(ErrorType::CorruptedFile(
            CorruptionType::IncorrectHeader
        )))
    } else {
        Ok(())
    }
}

/// Iterates over an encoded image to extract bytes from the last two bits of every pixel channel.
fn byte_iterator(image: &DynamicImage) -> impl Iterator<Item = u8> + '_ {
    let coord_iter = coord_iter(image.dimensions());

    let two_bit_iterator = coord_iter
        .map(|(x, y, channel)| image.get_pixel(x, y).channels()[channel as usize] & TWO_BIT_MASK);

    two_bit_iterator.batching(
        |iter| match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Some((a << 6) + (b << 4) + (c << 2) + d),
            _ => None,
        },
    )
}
