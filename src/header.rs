use iced::{widget::{row, toggler, text}, Element};

use crate::i18n::I18n;

pub struct Header {
  tree_mode: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
  OnTreeModeToggled(bool),
}

impl Header {
  pub fn new() -> Self {
    Self {
      tree_mode: false,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::OnTreeModeToggled(toggled) => {
        self.tree_mode = toggled
      }
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    let tree_mode_label = text(i18n.translate("header.tree_mode_label").to_string());

    let tree_mode_toggler = toggler(self.tree_mode)
      .on_toggle(|toggled| Message::OnTreeModeToggled(toggled));

    row![
      tree_mode_label,
      tree_mode_toggler
    ].into()
  }
}
