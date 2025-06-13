use iced::{widget::{container, text}, Element, Length};

use crate::{global_state::GlobalState, i18n::I18n};

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

  pub fn view(&self, i18n: &I18n, global_state: &GlobalState) -> Element<Message> {
    let tree_mode = global_state.tree_mode();
    container(text(format!("working area, tree mode: {tree_mode}")))
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
  }
}
