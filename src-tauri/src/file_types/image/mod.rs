use image::{io::Reader, DynamicImage, GenericImageView, Pixel};
use std::{fs::File, io::BufReader};

use crate::{Error, HEADER_BYTES};

mod decode;
mod encode;
pub use decode::decode;
pub use encode::encode;

/// The number of channels in a pixel that aren't an alpha channel.
const NON_ALPHA_CHANNELS: u8 = <DynamicImage as GenericImageView>::Pixel::CHANNEL_COUNT - 1;

/// The number of bits of data we can store per pixel in an image.
///
/// Every channel except for the alpha channel provides 2 bits.
const BITS_PER_PIXEL: u64 = NON_ALPHA_CHANNELS as u64 * 2;

/// Mask for the last two bits of a byte.
const TWO_BIT_MASK: u8 = 0b00000011;

/// Finds the amount of space in bytes, that can be used to store a secret file.
pub fn available_size_of(file: &File) -> Result<u64, Error> {
    let reader = Reader::new(BufReader::new(file)).with_guessed_format()?;
    let dimensions = reader.into_dimensions()?;

    let num_pixels = dimensions.0 as u64 * dimensions.1 as u64;
    Ok((num_pixels * BITS_PER_PIXEL) / 8 - HEADER_BYTES)
}

/// Iterates over the coordinates in an image in a deterministic order.
///
/// The first two u32 values are the x and y coordinate. The last value is which channel is next.
fn coord_iter(dimensions: (u32, u32)) -> impl Iterator<Item = (u32, u32, u8)> {
    let (width, height) = dimensions;
    (0..width).flat_map(move |x| {
        (0..height).flat_map(move |y| (0..NON_ALPHA_CHANNELS).map(move |c| (x, y, c)))
    })
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::Path};

    use crate::{
        file_types::image::{available_size_of, decode, encode},
        HEADER_BYTES,
    };

    #[test]
    fn size_of() {
        let path = Path::new("./test_data/stick.png");
        let file = File::open(path).unwrap();

        assert_eq!(available_size_of(&file).unwrap(), 98_304 - HEADER_BYTES);
    }

    #[test]
    fn can_encode_and_decode() {
        let base_image = File::open("./test_data/stick.png").unwrap();
        let secret_file = File::open("./test_data/story.txt").unwrap();
        let mut output_image = File::create("./test_data/stick_with_secret.png").unwrap();
        encode(&base_image, &secret_file, &mut output_image).unwrap();
        drop(output_image);

        let encoded_file = File::open("./test_data/stick_with_secret.png").unwrap();
        let mut output_file = File::create("./test_data/decoded_story.txt").unwrap();
        decode(&encoded_file, &mut output_file).unwrap();
        drop(output_file);

        let mut original_secret = Vec::new();
        let original_length = File::open("./test_data/story.txt")
            .unwrap()
            .read_to_end(&mut original_secret)
            .unwrap();
        let mut decoded_secret = Vec::new();
        let decoded_length = File::open("./test_data/decoded_story.txt")
            .unwrap()
            .read_to_end(&mut decoded_secret)
            .unwrap();

        assert_eq!(original_length, decoded_length);
        assert_eq!(original_secret, decoded_secret);
    }
}
