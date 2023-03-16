use std::{
    fs::File,
    io::{Read, Write, Seek},
};

/// Represents a type of file that we support encoding a message into.
#[derive(Clone, Copy)]
pub enum SupportedFileType {
    PNG,
    TXT,
}

impl SupportedFileType {
    /// Finds the file type from a file's name.
    ///
    /// Returns None if the file type is not supported.
    pub fn from_file_name(file_name: String) -> Option<SupportedFileType> {
        let file_name = file_name.to_lowercase();

        if file_name.ends_with(".png") {
            Some(SupportedFileType::PNG)
        } else if file_name.ends_with(".txt") {
            Some(SupportedFileType::TXT)
        } else {
            None
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
    pub fn new(file: File, file_name: String) -> Option<EncodedFile> {
        let file_type = SupportedFileType::from_file_name(file_name)?;

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
