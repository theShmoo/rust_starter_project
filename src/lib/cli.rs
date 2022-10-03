use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to read
    #[arg(short, long)]
    pub path: std::path::PathBuf,
}

pub fn parse() -> Cli {
    Cli::parse()
}
