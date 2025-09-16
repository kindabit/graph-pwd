use std::{collections::HashMap, error::Error, fmt::Display, fs};

use log::error;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Language {

  symbol: String,

  code: String,

}

impl Language {

  pub fn symbol(&self) -> &str {
    &self.symbol
  }

  pub fn code(&self) -> &str {
    &self.code
  }

}

impl Display for Language {

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.symbol, self.code)
  }

}

#[derive(Debug)]
pub struct I18n {
  translations: HashMap<String, String>,

  available_languages: Vec<Language>,

  current_language: String,
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

    let meta_string = fs::read_to_string(format!("./i18n/meta.yaml"))?;
    let meta_map: HashMap<String, serde_yml::Value> = serde_yml::from_str(&meta_string)?;

    let dir_iter = fs::read_dir("./i18n")?;
    let available_languages = dir_iter
      .filter_map(|result|
        match result {
          Ok(item) => {
            match item.file_name().to_str() {
              Some(file_name) => {
                if file_name != "meta.yaml" && file_name.ends_with(".yaml") {
                  // strip off extension name
                  let code = file_name.split_once('.').unwrap().0;
                  let symbol = meta_map
                    .get(&format!("{code}.language_symbol")).expect(&format!("Missing symbol for language {code}"))
                    .as_str().expect(&format!("The symbol of language {code} must be a string"));
                  Some(Language {
                    symbol: symbol.to_string(),
                    code: code.to_string(),
                  })
                }
                else {
                  None
                }
              },
              None => None,
            }
          }
          Err(err) => {
            error!("fail to read DirEntry: {err:?}");
            None
          }
        }
      )
      .collect();

    Ok(Self {
      translations,
      available_languages,
      current_language: language.to_string(),
    })
  }

  pub fn translate<'a>(&'a self, key: &'a str) -> String {
    match self.translations.get(key) {
      Some(text) => text.clone(),
      None => key.to_string(),
    }
  }

  pub fn translate_variable<'a>(&'a self, key: &'a str, variables: &[(&str, &str)]) -> String {
    match self.translations.get(key) {
      Some(text) => {
        let mut text = text.clone();
        for v in variables {
          text = text.replace(v.0, v.1);
        }
        text
      },
      None => key.to_string(),
    }
  }

  pub fn available_languages(&self) -> &[Language] {
    &self.available_languages[..]
  }

  pub fn current_language(&self) -> &String {
    &self.current_language
  }

}
