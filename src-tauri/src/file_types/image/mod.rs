use std::{fs::File, io::BufReader};

use image::{io::Reader, DynamicImage, GenericImageView, Pixel};

use crate::{error::ErrorType, file_types::supported_file::SupportedFile, HEADER_BYTES};

mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;

/// The number of channels in a pixel that aren't an alpha channel.
const NON_ALPHA_CHANNELS: u8 = <DynamicImage as GenericImageView>::Pixel::CHANNEL_COUNT - 1;

/// The number of bits we can store per pixel in an image.
///
/// Every channel except for the alpha channel provides two bits.
const BITS_PER_PIXEL: u64 = NON_ALPHA_CHANNELS as u64 * 2;

/// Mask for the last two bits of a byte.
const TWO_BIT_MASK: u8 = 0b00000011;

/// Finds the amount of space in bytes, that can be used to store a secret file.
pub fn available_size_of(file: &SupportedFile) -> std::result::Result<u64, ErrorType> {
    let reader = reader_from_supported_file(file);
    let dimensions = reader.into_dimensions()?;
    log::trace!("Read the size of an image");

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
    use std::{
        fs::File,
        io::Read,
        path::{Path, PathBuf},
    };

    use crate::{
        base_context, encoded_context,
        file_types::{
            image::{available_size_of, decode, encode},
            supported_file::SupportedFile,
        },
        output_context, secret_context, ErrorType, Result, HEADER_BYTES,
    };

    fn test_data_dir() -> PathBuf {
        "./test_data".into()
    }
    fn base_file() -> PathBuf {
        test_data_dir().join("stick.png")
    }
    fn encoded_file() -> PathBuf {
        test_data_dir().join("stick_with_secret.png")
    }
    fn secret_file() -> PathBuf {
        test_data_dir().join("story.txt")
    }

    fn assert_files_equal<P: AsRef<Path>>(
        file1: P,
        file2: P,
    ) -> std::result::Result<(), ErrorType> {
        let file1 = file_contents(file1)?;
        let file2 = file_contents(file2)?;

        assert_eq!(file1, file2);

        Ok(())
    }

    fn file_contents<P: AsRef<Path>>(file: P) -> std::result::Result<Vec<u8>, ErrorType> {
        let mut file_contents = vec![];
        File::open(file)?.read_to_end(&mut file_contents)?;

        Ok(file_contents)
    }

    #[test]
    fn size_of() -> std::result::Result<(), ErrorType> {
        let file = SupportedFile::open(base_file())?;

        assert_eq!(available_size_of(&file)?, 98_304 - HEADER_BYTES);

        Ok(())
    }

    #[test]
    fn can_encode() -> Result<()> {
        let output_image_path = test_data_dir().join("stick_encode.result.png");
        let base_image = base_context!(SupportedFile::open(base_file()))?;
        let secret_file = secret_context!(File::open(secret_file()))?;
        let mut output_image = output_context!(File::create(&output_image_path))?;
        encode(&base_image, &secret_file, &mut output_image)?;

        output_context!(assert_files_equal(output_image_path, encoded_file()))?;

        Ok(())
    }

    #[test]
    fn can_decode() -> Result<()> {
        let output_file_path = test_data_dir().join("decoded_story.result.txt");
        let encoded_file = encoded_context!(SupportedFile::open(encoded_file()))?;
        let mut output_file = output_context!(File::create(&output_file_path))?;
        decode(&encoded_file, &mut output_file)?;

        output_context!(assert_files_equal(output_file_path, secret_file()))?;

        Ok(())
    }
}
