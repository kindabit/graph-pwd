use iced::{widget::{button, container, row, text, toggler}, Element, Length};
use log::warn;

use crate::i18n::I18n;

pub struct WorkingArea {
}

#[derive(Clone, Debug)]
pub enum Message {
}

impl WorkingArea {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    container("working area")
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
  }
}
