use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub test: String,
}

pub fn parse(file: std::path::PathBuf) -> Config
{
  let content = std::fs::read_to_string(file).expect("Could not read file");
  let config : Config = toml::from_str(&content).expect("Could not parse file");
  config
}