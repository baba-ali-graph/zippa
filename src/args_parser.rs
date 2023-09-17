use clap::Parser;
// use zip::ZipWriter;

#[derive(Parser, Debug)]
#[clap(author = "Baba Ali", version = "0.1.0", about = "Zippa CLI Tool")]
pub struct ZippaArgs {
    #[clap(short = 's', long = "source")]
    pub source: String,

    #[clap(short = 'd', long = "dest")]
    pub dest: String,

    #[clap(short = 'c', long = "compression", default_value = "bzip2")]
    pub compression: String,

    #[clap(short = 'o', long = "override", takes_value = false)]
    pub over_ride: bool,
}

impl ZippaArgs {
    pub fn new() -> ZippaArgs {
        ZippaArgs::parse()
    }
}

