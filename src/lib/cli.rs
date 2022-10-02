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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let help = vec!["--help"];
        Cli::parse_from(help);

        let path = vec!["--path", "path/to/file.txt"];
        let out = Cli::parse_from(path);
        assert_eq!(out.path.to_str(), Some("path/to/file.txt"));

        let empty: Vec<&str> = Vec::new();
        Cli::parse_from(empty);
    }
}
