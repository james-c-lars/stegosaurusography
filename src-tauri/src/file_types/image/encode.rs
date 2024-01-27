use image::{GenericImage, GenericImageView, Pixel, Rgba};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error as IOError, Read},
};

use crate::{
    error::Error,
    file_types::{
        image::{coord_iter, reader_from_supported_file, TWO_BIT_MASK},
        supported_file::SupportedFile,
    },
};

/// Encodes the hidden file into the base image, and writes the results to the output image.
pub fn encode(
    base_image: &SupportedFile,
    secret_file: &File,
    output_image: &mut File,
) -> Result<(), Error> {
    // Getting the image we're going to encode with the secret
    log::trace!("Parsing the base image");
    let reader = reader_from_supported_file(base_image);
    let format = reader.format().expect("We just guessed the format");
    let mut image = reader.decode()?; // image is mut since we'll be editing it in place
    log::trace!("Parsed image: {image:?}");

    // Getting the data from the secret file. Reading it 2 bits at a time, as that's how much we can store in a pixel
    let secret_data = two_bit_iterator(secret_file)?;

    // Updating the image data in the buffer with the secret data
    let mut coord_iter = coord_iter(image.dimensions());

    for maybe_bits in secret_data {
        let (x, y, channel) = coord_iter
            .next()
            .expect("We only call encode on a secret file small enough to fully store");
        let secret_bits = maybe_bits?; // Reading in data from the secret file can fail

        // Updating the buffer
        let mut pixel: Rgba<u8> = image.get_pixel(x, y);
        let value = &mut pixel.channels_mut()[channel as usize];
        *value = (*value & !TWO_BIT_MASK) + secret_bits;
        image.put_pixel(x, y, pixel);
    }

    // Writing to the output file
    image.write_to(&mut BufWriter::new(output_image), format)?;
    Ok(())
}

/// Reads in a file 2 bits at a time. Will proceed the contents of the file with a u64 representation of the file size.
///
/// Returns an IO Error if it isn't able to read the size of a file from the metadata.
fn two_bit_iterator(
    secret_file: &File,
) -> Result<impl Iterator<Item = Result<u8, IOError>> + '_, IOError> {
    let header_bytes = secret_file
        .metadata()?
        .len()
        .to_be_bytes()
        .into_iter()
        .map(Ok);

    Ok(header_bytes
        .chain(BufReader::new(secret_file).bytes())
        .flat_map(|maybe_byte| {
            // bytes returns a Result as reading can fail at any time
            // If it does, we'll propagate the error
            match maybe_byte {
                Ok(byte) => vec![
                    Ok(byte >> 6 & TWO_BIT_MASK),
                    Ok(byte >> 4 & TWO_BIT_MASK),
                    Ok(byte >> 2 & TWO_BIT_MASK),
                    Ok(byte & TWO_BIT_MASK),
                ]
                .into_iter(),
                Err(err) => vec![Err(err)].into_iter(),
            }
        }))
}
