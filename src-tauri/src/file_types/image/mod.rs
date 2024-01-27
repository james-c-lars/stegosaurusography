use std::{fs::File, io::BufReader};

use image::{io::Reader, DynamicImage, GenericImageView, Pixel};

pub use decode::decode;
pub use encode::encode;

use crate::{Error, HEADER_BYTES};

use super::supported_file::SupportedFile;

mod decode;
mod encode;

/// The number of channels in a pixel that aren't an alpha channel.
const NON_ALPHA_CHANNELS: u8 = <DynamicImage as GenericImageView>::Pixel::CHANNEL_COUNT - 1;

/// The number of bits we can store per pixel in an image.
///
/// Every channel except for the alpha channel provides two bits.
const BITS_PER_PIXEL: u64 = NON_ALPHA_CHANNELS as u64 * 2;

/// Mask for the last two bits of a byte.
const TWO_BIT_MASK: u8 = 0b00000011;

/// Finds the amount of space in bytes, that can be used to store a secret file.
pub fn available_size_of(file: &SupportedFile) -> Result<u64, Error> {
    let reader = reader_from_supported_file(file);
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

/// Converts a SupportedFile into an ImageReader.
fn reader_from_supported_file(file: &SupportedFile) -> Reader<BufReader<&File>> {
    Reader::with_format(
        BufReader::new(file as &File),
        file.image_type().unwrap_or_else(|| {
            panic!(
                "We don't call image functions on a non-image. Actual file_type={:?}",
                file.file_type()
            )
        }),
    )
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::Path};
    use std::path::PathBuf;

    use crate::{
        file_types::{
            image::{available_size_of, decode, encode},
            supported_file::SupportedFile,
        },
        HEADER_BYTES,
    };

    fn test_data_dir() -> PathBuf { "./test_data".into() }
    fn base_file() -> PathBuf { test_data_dir().join("stick.png") }
    fn encoded_file() -> PathBuf { test_data_dir().join("stick_with_secret.png") }
    fn secret_file() -> PathBuf { test_data_dir().join("story.txt") }

    fn assert_files_equal<P: AsRef<Path>>(file1: P, file2: P) -> crate::Result<()> {
        let file1 = file_contents(file1)?;
        let file2 = file_contents(file2)?;

        assert_eq!(file1, file2);

        Ok(())
    }

    fn file_contents<P: AsRef<Path>>(file: P) -> crate::Result<Vec<u8>> {
        let mut file_contents = vec![];
        File::open(file)?.read_to_end(&mut file_contents)?;

        Ok(file_contents)
    }

    #[test]
    fn size_of() -> crate::Result<()> {
        let file = SupportedFile::open(base_file())?;

        assert_eq!(available_size_of(&file).unwrap(), 98_304 - HEADER_BYTES);

        Ok(())
    }

    #[test]
    fn can_encode() -> crate::Result<()> {
        let output_image_path = test_data_dir().join("stick_encode.result.png");
        let base_image = SupportedFile::open(base_file())?;
        let secret_file = File::open(secret_file())?;
        let mut output_image = File::create(&output_image_path)?;
        encode(&base_image, &secret_file, &mut output_image)?;

        assert_files_equal(output_image_path, encoded_file())?;

        Ok(())
    }

    #[test]
    fn can_decode() -> crate::Result<()> {
        let output_file_path = test_data_dir().join("decoded_story.result.txt");
        let encoded_file = SupportedFile::open("./test_data/stick_with_secret.png")?;
        let mut output_file = File::create(&output_file_path)?;
        decode(&encoded_file, &mut output_file)?;

        assert_files_equal(output_file_path, secret_file())?;

        Ok(())
    }
}
