use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Read, Write};
use std::path::Path;
use zip::{result::ZipResult, write::FileOptions, CompressionMethod, ZipWriter};

use crate::errors::ZippaError;
pub struct Zippa {
    dest_file: File,
    zip_writer: ZipWriter<File>,
}

impl Zippa {
    pub fn new(dest_file_path: &str) -> Zippa {
        let dest_file = File::create(Path::new(dest_file_path)).unwrap();
        Zippa {
            dest_file: dest_file.try_clone().unwrap(),
            zip_writer: ZipWriter::new(dest_file),
        }
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
        #[allow(deprecated)]
        self.zip_writer.start_file_from_path(file_path, options)?;
        file.read_to_end(&mut buf)?;
        self.zip_writer
            .write_all(&*buf)
            .map_err(|e| format!("An error occured while zipping: {}", e))?;
        println!("Done, {} zipped successfully", in_path.display());
        Ok(())
    }

    pub fn zipping_options(&self, method: CompressionMethod) -> FileOptions {
        return FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);
    }
    pub fn folder_zipping(
        &mut self,
        path: &Path,
        base_path: &Path,
        method: CompressionMethod,
    ) -> Result<(), Box<dyn Error>> {
        let options = self.zipping_options(method);
        #[allow(deprecated)]
        for item in std::fs::read_dir(&path)? {
            let i_path = item?.path();
            if i_path.is_dir() {
                let item_path_string = i_path.to_str().unwrap();
                println!("encountered another directory: {}", item_path_string);
                self.folder_zipping(&i_path, &base_path, method)?;
            } else {
                let file = File::open(&path);
                let rel_path = i_path.strip_prefix(&base_path).unwrap();
                self.zip_writer
                    .start_file(rel_path.to_str().unwrap(), options)?;
                // std::io::copy(&mut file, &mut self.zip_writer);
            }
        }
        // self.zip_writer.add_directory_from_path(path, options)?;
        Ok(())
    }

    pub fn compression_method(&self, method: &str) -> Result<CompressionMethod, ZippaError> {
        match method {
            "bzip2" => return Ok(CompressionMethod::Bzip2),
            "deflated" => return Ok(CompressionMethod::Deflated),
            "zstd" => return Ok(CompressionMethod::Zstd),
            _ => return Err(ZippaError::InvalidCompressionMethod),
        }
    }
}
