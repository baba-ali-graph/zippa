use clap::{self, Parser};

#[derive(Parser, Debug)]
#[clap(author = "Baba Ali", version = "0.1.0", about = "Zippa CLI Tool")]
pub enum SubCommand {
    #[clap(about = "Zipping files and folders")]
    Zap {
        #[clap(short = 's', long = "source")]
        source: String,

        #[clap(short = 'd', long = "dest")]
        dest: String,

        #[clap(short = 'c', long = "compression", default_value = "bzip2")]
        compression: String,

        #[clap(short = 'o', long = "override", takes_value = false)]
        over_ride: bool,
    },
    // #[clap(about = "Unzipping files and folders")]
    // UnzipAction
}

#[derive(Parser, Debug)]
#[clap(author = "Baba Ali", version = "0.1.0", about = "Zippa CLI Tool")]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}
