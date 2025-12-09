use serde::Deserialize;
use std::path::Path;
use tokio::fs;

const DEFAULT_FP_RED_CARDS: &str = "./assets/original/redApples.txt";
const DEFAULT_FP_GREEN_CARDS: &str = "./assets/original/greenApples.txt";

struct FileConfig {}

#[non_exhaustive]
#[derive(Deserialize)]
pub struct Config {
    key: u32,
}

impl Config {
    pub fn get(&self) -> u32 {
        self.key
    }
}

pub async fn parse_config(path: impl AsRef<Path>) -> anyhow::Result<Config> {
    let contents = fs::read_to_string(path).await?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
