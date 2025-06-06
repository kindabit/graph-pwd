use iced::{widget::{button, row, text, toggler}, Element};

use crate::i18n::I18n;

pub struct Header {
  tree_mode: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
  OnTreeModeToggled(bool),
  OnNewButtonClicked,
  OnLoadButtonClicked,
  OnSaveButtonClicked,
  OnSaveAsButtonClicked,
}

impl Header {
  pub fn new() -> Self {
    Self {
      tree_mode: false,
    }
  }

  pub fn update(&mut self, message: Message) -> Option<crate::Message> {
    match message {
      Message::OnTreeModeToggled(toggled) => {
        self.tree_mode = toggled;
        None
      }
      Message::OnNewButtonClicked => {
        Some(crate::Message::NewDatabase)
      }
      Message::OnLoadButtonClicked => {
        Some(crate::Message::LoadDatabase)
      }
      Message::OnSaveButtonClicked => {
        Some(crate::Message::SaveDatabase)
      }
      Message::OnSaveAsButtonClicked => {
        Some(crate::Message::SaveAsDatabase)
      }
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    let tree_mode_label = text(i18n.translate("header.tree_mode_label"));

    let tree_mode_toggler = toggler(self.tree_mode)
      .on_toggle(|toggled| Message::OnTreeModeToggled(toggled));

    let new_button = button(text(i18n.translate("header.new_button")))
      .on_press(Message::OnNewButtonClicked);

    let load_button = button(text(i18n.translate("header.load_button")))
      .on_press(Message::OnLoadButtonClicked);

    let save_button = button(text(i18n.translate("header.save_button")))
      .on_press(Message::OnSaveButtonClicked);

    let save_as_button = button(text(i18n.translate("header.save_as_button")))
      .on_press(Message::OnSaveAsButtonClicked);

    row![
      tree_mode_label,
      tree_mode_toggler,
      new_button,
      load_button,
      save_button,
      save_as_button,
    ].into()
  }
}
