use crate::errors::ZippaError;
use std::error::Error;
use std::fs;
use std::fs::File;

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub struct Zippa {
    pub dest_path: PathBuf,
    pub zip_writer: ZipWriter<File>,
}

impl Zippa {
    pub fn new(dest_file_path: &Option<String>) -> Result<Self, io::Error> {
        let dest_path: PathBuf;
        if let Some(dest_string) = dest_file_path {
            dest_path = PathBuf::from(dest_string);
        } else {
            dest_path = PathBuf::from("./output.zip");
        }
        let dest_file = File::create(&dest_path)?;
        let zip_writer = ZipWriter::new(dest_file.try_clone()?);
        Ok(Zippa {
            dest_path,
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
        if self.dest_path.is_dir() {
            panic!("Expected destination path as an archive file")
        }

        let file_path = Path::new(in_path);
        let options = self.zipping_options(method);
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(&file_path)?;
        self.zip_writer
            .start_file(file_path.to_string_lossy().into_owned(), options)?;
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
        if self.dest_path.is_dir() {
            panic!("Expected destination path as an archive file")
        }
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

    pub fn unzip_archive(&self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut curr_file = archive.by_index(i)?;
            let f_name = curr_file.name();
            let output_path = PathBuf::from(&f_name);

            if curr_file.is_dir() {
                fs::create_dir_all(&output_path)?;
            } else {
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(&parent)?;
                }
                let mut output_file = File::create(&output_path)?;
                io::copy(&mut curr_file, &mut output_file)?;
            }
        }
        Ok(())
    }
}
