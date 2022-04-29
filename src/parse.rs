use anyhow::{anyhow, Result};
use std::{fs::File, path::Path};

use crate::config::data::*;

pub fn parse_solana_config() -> Option<SolanaConfig> {
    let home = match home::home_dir() {
        Some(path) => path,
        None => panic!("Unsupported OS!"),
    };

    let config_path = Path::new(&home)
        .join(".config")
        .join("solana")
        .join("cli")
        .join("config.yml");

    let conf_file = match File::open(config_path) {
        Ok(f) => f,
        Err(_) => return None,
    };
    serde_yaml::from_reader(&conf_file).ok()
}

pub fn path_to_string(path: &Path) -> Result<String> {
    match path.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow!("Couldn't convert path to string.")),
    }
}
