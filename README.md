Here is a suggested README.md file for your Rust terminal zipping application:

# Zippa

Zippa is a command line utility for zipping folders and files using Rust.

## Usage

### Zipping Files and Folders

```bash
zippa zap -s FOLDER_TO_ZIP -d OUTPUT.zip
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

If not passed, the compression option defaults to `bzip2` and the `--override` flag is applicable when you want it to  overwrite existing output files.


### Unzipping Files and Folders

```bash
zippa unzap -s FOLDER_TO_ZIP -d OUTPUT.zip
```

The following flags are available:

`-s, --source FOLDER`    
The source folder to zip  

`-d, --dest FILE`    
The output directory destination  


## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
zippa = "*"
``` 

And run `cargo install --path .` to install.

## Contributing

Pull requests are welcome!