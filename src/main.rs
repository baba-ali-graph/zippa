use std::path::Path;
use zippa::cli::{Cli, SubCommand};
use zippa::zippa::Zippa;

fn main() {
    let cli = Cli::new();

    match cli.sub_command {
        SubCommand::Zap {
            source,
            dest,
            compression,
            overwrite,
        } => {
            let src_path = Path::new(&source);
            let mut zippa = Zippa::new(&dest).unwrap();
            let method = zippa
                .compression_method(&compression)
                .unwrap_or_else(|_err| {
                    panic!("Invalid compression method received, exiting...");
                });

            if src_path.is_file() {
                match zippa.file_zipping(&src_path, method) {
                    Ok(_) => println!("File  zipped successfully"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if src_path.is_dir() {
                match zippa.folder_zipping(&src_path, &src_path, method) {
                    Ok(_) => println!("Folder zipped successfully"),
                    Err(e) => eprintln!("Error: {}", e),
                };
            } else {
                println!("Unknown source type!!")
            }
        }
        SubCommand::Unzap {
            source,
            dest,
            overwrite,
        } => {
            let mut zippa = Zippa::new(&dest).unwrap();
            match zippa.unzip_archive(&source) {
                Ok(()) => println!("Archive unzipped successfully"),
                Err(e) => eprint!("An error occured while unzipping: {:?}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_are_parsed_correctly() {
        let source = "baba";
        let dest = "temp";
        let _matches = ["ts", "--source", source, "--dest", dest];
        let sub_command = SubCommand::Zap {
            source: String::from(source),
            dest: String::from(dest),
            compression: String::from(source),
            overwrite: false,
        };
        let cli = Cli {
            sub_command: sub_command,
        };

        let mut is_same = false;

        if let SubCommand::Zap {
            source,
            dest,
            compression,
            overwrite,
        } = cli.sub_command
        {
            is_same = source == source;
        }

        assert_eq!(is_same, true);
    }
}
