use crate::config::Config;
use std::fs;

pub fn load_config(path: &str) -> Result<Config, String> {
    let data = fs::read_to_string(path)
        .map_err(|e| format!("failed to read config {}: {}", path, e))?;

    toml::from_str(&data)
        .map_err(|e| format!("failed to parse config {}: {}", path, e))
}
