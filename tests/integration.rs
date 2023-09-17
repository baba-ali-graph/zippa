extern crate zippa;

use std::fs::{self, File};
use std::path::Path;
use zip::CompressionMethod;
use std::io::{Write, Read};
use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_new_zippa_instance() {
        let temp_zip_file = NamedTempFile::new().unwrap();
        let zippa_result = Zippa::new(temp_zip_file.path().to_str().unwrap());
        if let Ok(zippa) = zippa_result {
            assert!(zippa.dest_file.metadata().is_ok());
            assert!(zippa.is_compression_method_supported("bzip2"));
        }
    }

    #[test]
    fn test_new_zippa_instance_with_invalid_file_path() {
        let invalid_path = "/this/path/does/not/exist.zip";
        let zippa_result = Zippa::new(invalid_path);
        if let Ok(zippa) = zippa_result {
        assert!(zippa.dest_file.metadata().is_err());
        }
    }

    #[test]
    fn test_file_zipping() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_zip_file = NamedTempFile::new().unwrap();

        let mut file = File::create(temp_file.path()).unwrap();
        file.write_all(b"Hello world!").unwrap();

        let zippa_result = Zippa::new(temp_zip_file.path().to_str().unwrap());
        if let Ok(mut zippa) = zippa_result {
            zippa.file_zipping(temp_file.path(), CompressionMethod::Deflated).unwrap();

            let mut zip_file = File::open(temp_zip_file.path()).unwrap();
            let mut archive = zip::ZipArchive::new(&mut zip_file).unwrap();
            let mut zip_file_contents = String::new();
            archive.by_index(0).unwrap().read_to_string(&mut zip_file_contents).unwrap();

            assert_eq!(zip_file_contents, "Hello world!");
        }
    }

    #[test]
    #[should_panic(expected = "No file/folder found at specified path")]
    fn test_file_zipping_with_nonexistent_file() {
        let temp_zip_file = NamedTempFile::new().unwrap();
        let  zippa_result = Zippa::new(temp_zip_file.path().to_str().unwrap());
        if let Ok(mut zippa) = zippa_result {
        zippa.file_zipping(Path::new("/this/path/does/not/exist"), CompressionMethod::Deflated).unwrap();
        }
    }

    #[test]
    fn test_folder_zipping() {
        let temp_dir = tempdir().unwrap();
        let temp_zip_file = NamedTempFile::new().unwrap();

        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("dir1/file2.txt");
        let file3_path = temp_dir.path().join("dir1/dir2/file3.txt");

        fs::write(&file1_path, "file1 contents").unwrap();
        fs::create_dir(temp_dir.path().join("dir1")).unwrap();
        fs::write(&file2_path, "file2 contents").unwrap();
        fs::create_dir(temp_dir.path().join("dir1/dir2")).unwrap();
        fs::write(&file3_path, "file3 contents").unwrap();

        let zippa_result = Zippa::new(temp_zip_file.path().to_str().unwrap());
        if let Ok(mut zippa) = zippa_result {
        
            zippa.folder_zipping(temp_dir.path(), temp_dir.path(), CompressionMethod::Deflated).unwrap();

            let mut zip_file = File::open(temp_zip_file.path()).unwrap();
            let mut archive = zip::ZipArchive::new(&mut zip_file).unwrap();

            assert_eq!(archive.len(), 3);

            let mut file1 = archive.by_index(0).unwrap();
            let mut file1_contents = String::new();
            file1.read_to_string(&mut file1_contents).unwrap();


            assert_eq!(file1.name(), "file1.txt");
            assert_eq!(file1_contents, "file1 contents");
} 
}

    #[test]
    fn test_folder_zipping_with_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let temp_zip_file = NamedTempFile::new().unwrap();

        let maybe_zippa = Zippa::new(temp_zip_file.path().to_str().unwrap());
        
        if let Ok(mut zippa) = maybe_zippa {
            zippa.folder_zipping(temp_dir.path(), temp_dir.path(), CompressionMethod::Deflated).unwrap();
        }
    }

    #[test]   
     fn test_folder_zipping_with_nonexistent_directory() {
        let temp_zip_file = NamedTempFile::new().unwrap();
        let  maybe_zippa = Zippa::new(temp_zip_file.path().to_str().unwrap());
        if let Ok(mut zippa) = maybe_zippa {
        assert!(zippa.folder_zipping(Path::new("/this/path/does/not/exist"), Path::new("."), CompressionMethod::Deflated).is_err());
        }
     }

    #[test]
    fn test_zipping_options() {
        let maybe_zippa = Zippa::new("test.zip");
        if let Ok(mut zippa) = maybe_zippa {
            let options_bzip2 = zippa.compression_method("bzip2").unwrap();
            let options_deflated = zippa.compression_method("deflated").unwrap();
            let options_zstd = zippa.compression_method("zstd").unwrap();
            
            assert_eq!(options_bzip2, CompressionMethod::Bzip2);
            assert_eq!(options_deflated, CompressionMethod::Deflated);
            assert_eq!(options_zstd, CompressionMethod::Zstd);
        }
    }

    // #[test]
    // fn test_compression_method() {
    //     let zippa = Zippa::new("test.zip");

    //     let method_bzip2 = zippa.compression_method("bzip2").unwrap();
    //     let method_deflated = zippa.compression_method("deflated").unwrap();
    //     let method_zstd = zippa.compression_method("zstd").unwrap();

    //     assert_eq!(method_bzip2, CompressionMethod::Bzip2);
    //     assert_eq!(method_deflated, CompressionMethod::Deflated);
    //     assert_eq!(method_zstd, CompressionMethod::Zstd);

    //     assert!(zippa.compression_method("invalid").is_err());
    // }
