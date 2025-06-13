use iced::{widget::{button, column, row, text}, Element};

use crate::i18n::I18n;

pub struct ConfirmDialog {

  id: usize,

  title: String,

  content: String,

  on_confirm_message: crate::Message,

  on_cancel_message: crate::Message,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnConfirmButtonClicked(usize),

  OnCancelButtonClicked(usize),

}

impl ConfirmDialog {

  pub fn new(
    id: usize,
    title: String,
    content: String,
    on_confirm_message: crate::Message,
    on_cancel_message: crate::Message,
  ) -> Self {
    Self {
      id,
      title,
      content,
      on_confirm_message,
      on_cancel_message,
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    let title = text(&self.title);

    let content = text(&self.content);

    let confirm_button = button(text(i18n.translate("confirm_dialog.confirm_button")))
      .on_press(Message::OnConfirmButtonClicked(self.id));

    let cancel_button = button(text(i18n.translate("confirm_dialog.cancel_button")))
      .on_press(Message::OnCancelButtonClicked(self.id));

    column![
      title,
      content,
      row![
        confirm_button,
        cancel_button,
      ],
    ].into()
  }

  pub fn into_on_confirm_message(self) -> crate::Message {
    self.on_confirm_message
  }

  pub fn into_on_cancel_message(self) -> crate::Message {
    self.on_cancel_message
  }

}
