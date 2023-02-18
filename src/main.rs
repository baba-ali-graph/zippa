mod args_parser;
mod errors;
mod zippa;

use args_parser::ZippaArgs;
use errors::ZippaError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{
    result::{ZipError, ZipResult},
    write::FileOptions,
    CompressionMethod, ZipWriter,
};
use zippa::Zippa;

fn main() {
    let args = ZippaArgs::new();
    let src_path = Path::new(&args.source);
    
    let mut zippa = Zippa::new(&args.dest);
    let method = zippa.compression_method(args.compression).unwrap_or_else(|err| {
        println!("Invalid compression method received, exiting...");
        std::process::abort(0);
    })

    if path.is_file() {
        match zippa.file_zipping(src_path, method) {
                    Ok(path) => println!("File {} zipped successfully", path),
                    Err(e) => eprintln!("Error: {}", e),
        }
    } else if path.is_dir() {
                #[allow(deprecated)]
                match zip_writer.add_directory_from_path(path, options) {
                    Ok(rep) => println!("Folder {} zipped successfully", args.source),
                    Err(e) => eprintln!("Error: {}", e),
                };
            } else {
                println!("Unknown source type!!")
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
) -> ZipResult<&'a str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_are_parsed_correctly() {
        let source = "baba";
        let dest = "temp";
        let matches = ["ts", "--source", source, "--dest", dest];
        let app = ZippaArgs {
            source: String::from(source),
            dest: String::from(dest),
            compression: String::from(source),
            over_ride: false,
        };

        assert_eq!(app.source, source);
    }
}
