Here is a suggested README.md file for your Rust terminal zipping application:

# Zippa

Zippa is a command line utility for zipping folders and files using Rust.

## Usage

```bash
zippa -s FOLDER_TO_ZIP -d OUTPUT.zip
```

The following flags are available:

`-s, --source FOLDER`    
The source folder to zip  

`-d, --dest FILE`    
The output zip file  

`-c, --compression`    
The compression algorithm to use. Supported values are `bzip2` (default), `gzip` or `zip`   

`-o, --override`     
Override existing output file

Based on your Rust code snippet, the compression option defaults to `bzip2` and the `--override` flag can be used to overwrite existing output files.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
zippa = "*"
``` 

And run `cargo install --path .` to install.

## Contributing

Pull requests are welcome!