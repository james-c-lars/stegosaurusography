use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use image::{io::Reader, DynamicImage, GenericImageView, Pixel};
use itertools::Itertools;

use crate::{
    file_types::image::{coord_iter, TWO_BIT_MASK},
    CorruptionType, Error, HEADER_BYTES,
};

/// Decodes the encoded image, and writes the results to the output file.
pub fn decode(encoded_image: &File, output_file: &mut File) -> Result<(), Error> {
    // Getting the image that contains the secret
    let reader = Reader::new(BufReader::new(encoded_image)).with_guessed_format()?;
    let image = reader.decode()?;

    // Starting by reading the header to find the original file size
    let mut secret_data = byte_iterator(&image);
    let mut header_bytes = [0; HEADER_BYTES as usize];
    for header_byte in header_bytes.iter_mut() {
        match secret_data.next() {
            Some(byte) => *header_byte = byte,
            None => return Err(Error::CorruptedFile(CorruptionType::FileTooSmallForHeader)),
        }
    }
    let file_size = u64::from_be_bytes(header_bytes);

    // Writing out the secret data to the output file
    let mut writer = BufWriter::new(output_file);
    let mut bytes_written = 0;
    for byte in secret_data.take(file_size as usize) {
        writer.write(&[byte])?;
        bytes_written += 1;
    }

    // Verifying that we were able to read the entire file according to the header
    if bytes_written < file_size {
        Err(Error::CorruptedFile(CorruptionType::IncorrectHeader))
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
