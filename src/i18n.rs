use std::{collections::HashMap, error::Error, fs};

use log::error;

use crate::config::Config;

#[derive(Debug)]
pub struct I18n {
  translations: HashMap<String, String>,

  available_languages: Vec<String>,
}

impl I18n {
  pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
    let language = config.get_language();

    let translations_string = fs::read_to_string(format!("./i18n/{language}.yaml"))?;
    let translations_map: HashMap<String, serde_yml::Value> = serde_yml::from_str(&translations_string)?;
    let translations = translations_map
      .into_iter()
      .map(|(key, value)| {
        if let Some(value) = value.as_str() {
          (key, value.to_string())
        }
        else {
          let key_cloned = key.clone();
          (key, key_cloned)
        }
      })
      .collect();

    let dir_iter = fs::read_dir("./i18n")?;
    let available_languages = dir_iter
      .filter_map(|result|
        match result {
          Ok(item) =>
            match item.file_name().to_str() {
              Some(file_name) => Some(file_name.to_string()),
              None => None,
            },
          Err(err) => {
            error!("fail to read DirEntry: {err:?}");
            None
          },
        }
      )
      .collect();

    Ok(Self {
      translations,
      available_languages,
    })
  }

  pub fn translate<'a>(&'a self, key: &'a str) -> &'a str {
    match self.translations.get(key) {
      Some(text) => text,
      None => key,
    }
  }

  pub fn get_available_languages(&self) -> &[String] {
    &self.available_languages[..]
  }
}
