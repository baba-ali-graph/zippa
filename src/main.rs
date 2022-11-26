mod args_parser;
mod errors;

use args_parser::zippa_args;
use errors::ZippaError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{
    result::{ZipError, ZipResult},
    write::FileOptions,
    CompressionMethod, ZipWriter,
};

fn main() {
    let args = zippa_args();
    let path = Path::new(&args.source);

    if !path.exists() {
        panic!("No file/folder found at specified path");
    }

    let out_file = File::create(Path::new(&args.dest)).unwrap();
    let mut zip_writer = ZipWriter::new(out_file);

    println!("args are {:?}", args);
    let maybe_method = compression_method(&args.compression);
    println!("{:?}", maybe_method);
    match maybe_method {
        Ok(method) => {
            let options = FileOptions::default()
                .compression_method(method)
                .unix_permissions(0o755);
            if path.is_file() {
                match file_zip(&mut zip_writer, &args.source, options) {
                    Ok(path) => println!("File {} zipped successfully", path),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if path.is_dir() {
                #[allow(deprecated)]
                match zip_writer.add_directory_from_path(path, options) {
                    Ok(resp) => println!("Folder {} zipped successfully", args.source),
                    Err(e) => eprintln!("Error: {}", e),
                };
            } else {
                println!("Unknown source type!!")
            }
        }
        Err(e) => {
            eprintln!("Err: {}", e)
        }
    }
}

fn compression_method(method: &str) -> Result<CompressionMethod, ZippaError> {
    match method {
        "bzip2" => return Ok(CompressionMethod::Bzip2),
        "deflated" => return Ok(CompressionMethod::Deflated),
        "zstd" => return Ok(CompressionMethod::Zstd),
        _ => return Err(ZippaError::InvalidCompressionMethod),
    }
}

fn file_zip<'a>(
    zip_writer: &mut ZipWriter<File>,
    str_path: &'a str,
    options: FileOptions,
) -> ZipResult<(&'a str)> {
    let path = Path::new(str_path);
    let mut buf: Vec<u8> = Vec::new();
    #[allow(deprecated)]
    zip_writer.start_file_from_path(path, options)?;
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    zip_writer.write_all(&*buf)?;
    buf.clear();
    Ok(str_path)
}

// zip.add_directory_from_path(path, options) -> options = FileOptions::default().compression_method(method).unix
