use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  language: String,
  tree_mode: bool,
  clear_clipboard_countdown: i32,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let config_string = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yml::from_str(&config_string)?;
    Ok(config)
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let config_string = serde_yml::to_string(self)?;
    fs::write("./config.yaml", config_string)?;
    Ok(())
  }

  pub fn get_language(&self) -> &str {
    &self.language
  }

  pub fn set_language(&mut self, val: &str) {
    self.language = val.to_string()
  }

  pub fn tree_mode(&self) -> bool {
    self.tree_mode
  }

  pub fn set_tree_mode(&mut self, tree_mode: bool) {
    self.tree_mode = tree_mode;
  }

  pub fn clear_clipboard_countdown(&self) -> i32 {
    self.clear_clipboard_countdown
  }

  pub fn set_clear_clipboard_countdown(&mut self, clear_clipboard_countdown: i32) {
    self.clear_clipboard_countdown = clear_clipboard_countdown;
  }

}
