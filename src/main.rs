use std::path::Path;
use zippa::core::Zippa;
use zippa::zippa_app::ZippaApp;

fn main() {
    let args = ZippaApp::new();
    let src_path = Path::new(&args.source);
    let mut zippa = Zippa::new(&args.dest).unwrap();
    let method = zippa
        .compression_method(&args.compression)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_are_parsed_correctly() {
        let source = "baba";
        let dest = "temp";
        let _matches = ["ts", "--source", source, "--dest", dest];
        let app = ZippaApp {
            source: String::from(source),
            dest: String::from(dest),
            compression: String::from(source),
            over_ride: false,
        };

        assert_eq!(app.source, source);
    }
}
