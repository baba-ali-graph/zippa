use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Baba Ali", version = "0.1.0", about = "Zippa CLI Tool")]
pub struct ZippaArguments {
    #[clap(short = 's', long = "source")]
    pub source: String,

    #[clap(short = 'd', long = "dest")]
    pub dest: String,

    #[clap(short = 'c', long = "compression", default_value = "bzip2")]
    pub compression: String,

    #[clap(short = 'o', long = "override", takes_value = false)]
    pub over_ride: bool,
}

pub fn zippa_args() -> ZippaArguments {
    return ZippaArguments::parse();
}
