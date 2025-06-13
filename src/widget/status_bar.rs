use iced::{widget::{button, container, row, text, toggler}, Alignment, Element, Length};
use log::warn;

use crate::{database::Database, i18n::I18n};

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

  pub fn view(&self, i18n: &I18n, db: Option<&Database>) -> Element<Message> {
    let mut current_database_string = i18n.translate("status_bar.current_database");
    let current_database_path_string = if let Some(db) = db {
      db.path().to_string()
    } else {
      i18n.translate("status_bar.no_opened_database")
    };
    current_database_string.push_str(&current_database_path_string);
    let current_database_text = text(current_database_string);

    row![
      current_database_text,
    ]
    .padding([6, 12])
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
  }
}
