use errors::ZippaError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{result::ZipResult, write::FileOptions, CompressionMethod, ZipWriter};

pub struct Zippa {
    dest_file: File,
    zip_writer: ZipWriter<File>,
}

impl Zippa {
    pub fn new(dest_file_path: &str) -> Zippa {
        let dest_file = File::create(Path::new(dest_file_path)).unwrap();
        let mut zip_writer = ZipWriter::new(dest_file);
        Zippa {
            dest_file,
            zip_writer,
        }
    }
    pub fn file_zipping(&self, &in_path: Path, method: CompressionMethod) -> Result<(), str> {
        if !in_path.exists() {
            return Err("No file/folder found at specified path");
        }
        let file_path = Path::new(in_path);
        let options = self.zipping_options(method);
        let mut buf: Vec<u8> = Vec::new();
        let mut file = File::open(&file_path)?;
        #[allow(deprecated)]
        self.zip_writer.start_file_from_path(file_path, options);
        file.read_to_end(&mut buf)?;
        self.zip_writer.write_all(&*buf);
        println!("Done, {} zipped successfully", in_path);
        Ok(&());
    }

    pub fn zipping_options(&self, method: CompressionMethod) -> FileOptions {
        return FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);
    }

    pub fn compression_method(method: &str) -> Result<CompressionMethod, ZippaError> {
        match method {
            "bzip2" => return Ok(CompressionMethod::Bzip2),
            "deflated" => return Ok(CompressionMethod::Deflated),
            "zstd" => return Ok(CompressionMethod::Zstd),
            _ => return Err(ZippaError::InvalidCompressionMethod),
        }
    }
}
