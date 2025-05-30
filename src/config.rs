use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  language: String,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let config_string = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yml::from_str(&config_string)?;
    Ok(config)
  }

  pub fn get_language(&self) -> &str {
    &self.language
  }

  pub fn set_language(&mut self, val: &str) {
    self.language = val.to_string()
  }
}
