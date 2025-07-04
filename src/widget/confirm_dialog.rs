use iced::{widget::{Button, Column, Row, Text}, Element};

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

  OnConfirmButtonPress(usize),

  OnCancelButtonPress(usize),

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
    let title = Text::new(&self.title);

    let content = Text::new(&self.content);

    let confirm_button = Button::new(Text::new(i18n.translate("confirm_dialog.confirm_button")))
      .on_press(Message::OnConfirmButtonPress(self.id));

    let cancel_button = Button::new(Text::new(i18n.translate("confirm_dialog.cancel_button")))
      .on_press(Message::OnCancelButtonPress(self.id));

    Column::new()
    .push(title)
    .push(content)
    .push(
      Row::new()
      .push(confirm_button)
      .push(cancel_button)
    )
    .into()
  }

  pub fn into_on_confirm_message(self) -> crate::Message {
    self.on_confirm_message
  }

  pub fn into_on_cancel_message(self) -> crate::Message {
    self.on_cancel_message
  }

}
