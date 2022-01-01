use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordConfig {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub discord: DiscordConfig,
}

impl Config {
    /// ファイルパスを指定して設定を読み込む
    pub fn try_from_path(path: &str) -> Result<Self> {
        use std::{fs::File, io::Read};
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer)?;
        let config = toml::from_str::<Config>(&buffer)?;
        Ok(config)
    }
}
