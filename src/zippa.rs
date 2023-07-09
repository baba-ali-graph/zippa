use crate::errors::ZippaError;
use std::error::Error;
use std::fs::File;

use std::io::{self, Read, Write};
use std::path::{Path};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub struct Zippa {
    pub dest_file: File,
    pub zip_writer: ZipWriter<File>,
}

impl Zippa {
    pub fn new(dest_file_path: &str) -> Result<Self, io::Error> {
        let dest_file = File::create(Path::new(dest_file_path))?;
        let zip_writer = ZipWriter::new(dest_file.try_clone()?);
        Ok(Zippa {
            dest_file,
            zip_writer,
        })
    }

    pub fn file_zipping(
        &mut self,
        in_path: &Path,
        method: CompressionMethod,
    ) -> Result<(), Box<dyn Error>> {
        if !in_path.exists() {
            panic!("No file/folder found at specified path");
        }
        let file_path = Path::new(in_path);
        let options = self.zipping_options(method);
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(&file_path)?;
        self.zip_writer.start_file(file_path.to_string_lossy().into_owned(), options)?;
        file.read_to_end(&mut buf)?;
        self.zip_writer
            .write_all(&*buf)
            .map_err(|e| format!("An error occurred while zipping: {}", e))?;
        println!("Done, {} zipped successfully", in_path.display());
        Ok(())
    }

    pub fn zipping_options(&self, method: CompressionMethod) -> FileOptions {
        FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755)
    }

    pub fn folder_zipping(
        &mut self,
        path: &Path,
        base_path: &Path,
        method: CompressionMethod,
    ) -> Result<(), Box<dyn Error>> {
        let options = self.zipping_options(method);
        for item in std::fs::read_dir(&path)? {
            let i_path = item?.path();
            if i_path.is_dir() {
                self.folder_zipping(&i_path, base_path, method)?;
            } else {
                let file_path = i_path.strip_prefix(&base_path)?;
                let mut file = File::open(&i_path)?;
                self.zip_writer
                    .start_file(file_path.to_string_lossy().into_owned(), options)?;
                io::copy(&mut file, &mut self.zip_writer)?;
            }
        }
        Ok(())
    }

    pub fn compression_method(&self, method: &str) -> Result<CompressionMethod, ZippaError> {
        match method {
            "bzip2" => Ok(CompressionMethod::Bzip2),
            "deflated" => Ok(CompressionMethod::Deflated),
            "zstd" => Ok(CompressionMethod::Zstd),
            _ => Err(ZippaError::InvalidCompressionMethod),
        }
    }

    pub fn is_compression_method_supported(&self, method: &str) -> bool {
        self.compression_method(method).is_ok()
    }
    
}
