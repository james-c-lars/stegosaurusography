use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
};

/// Represents a type of file that we support encoding a message into.
#[derive(Clone, Copy)]
pub enum SupportedFileType {
    PNG,
}

impl SupportedFileType {
    /// Finds the file type from a file's name.
    ///
    /// Returns None if the file type is not supported.
    pub fn from_file_path(file_path: PathBuf) -> Option<SupportedFileType> {
        match file_path.extension()?.to_ascii_lowercase().to_str()? {
            "png" => Some(SupportedFileType::PNG),
            _ => None,
        }
    }
}

/// A wrapper for File that includes what type of file it is.
pub struct EncodedFile {
    file: File,
    file_type: SupportedFileType,
}

impl EncodedFile {
    /// Constructs a new EncodedFile.
    pub fn new(file: File, file_path: PathBuf) -> Option<EncodedFile> {
        let file_type = SupportedFileType::from_file_path(file_path)?;

        Some(EncodedFile { file, file_type })
    }

    /// Gets the file type of the EncodedFile.
    pub fn file_type(&self) -> SupportedFileType {
        self.file_type
    }
}

impl Read for EncodedFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for EncodedFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.file.seek(pos)
    }
}

impl Write for EncodedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
