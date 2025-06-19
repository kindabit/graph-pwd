use std::sync::{Arc, Mutex};

use iced::{widget::{row, text}, Alignment, Element, Length};

use crate::{database::Database, i18n::I18n, style_variable::{StyleVariable}};

pub struct StatusBar {
}

#[derive(Clone, Debug)]
pub enum Message {
}

impl StatusBar {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
    }
  }

  pub fn view(&self, i18n: &I18n, db: Option<&Database>, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut current_database_string = i18n.translate("status_bar.current_database");
    let current_database_path_string = if let Some(db) = db {
      db.path().to_string()
    } else {
      i18n.translate("status_bar.no_opened_database")
    };
    current_database_string.push_str(&current_database_path_string);
    let current_database_text = text(current_database_string);

    let style_variable = StyleVariable::lock(style_variable);

    row![
      current_database_text,
    ]
    .padding(style_variable.status_bar_padding)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
  }
}
