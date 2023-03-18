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
    let method = zippa
        .compression_method(&args.compression)
        .unwrap_or_else(|err| {
            panic!("Invalid compression method received, exiting...");
        });

    if src_path.is_file() {
        match zippa.file_zipping(src_path, method) {
            Ok(res) => println!("File  zipped successfully"),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if src_path.is_dir() {
        match zippa.folder_zipping(src_path, method) {
            Ok(res) => println!("Folder zipped successfully"),
            Err(e) => eprintln!("Error: {}", e),
        };
    } else {
        println!("Unknown source type!!")
    }
}

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
