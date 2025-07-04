use iced::{widget::{Button, Column, Text}, Element};

use crate::i18n::I18n;

pub enum PopupDialogType {

  Info,

  Success,

  Warning,

  Error,

}

pub struct PopupDialog {

  id: usize,

  title: String,

  content: String,

  r#type: PopupDialogType,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnOkButtonPress(usize),

}

impl PopupDialog {

  pub fn new(id: usize, title: impl Into<String>, content: impl Into<String>, r#type: PopupDialogType) -> Self {
    Self {
      id,
      title: title.into(),
      content: content.into(),
      r#type,
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    let title = Text::new(&self.title);

    let content = Text::new(&self.content);

    let ok_button = Button::new(Text::new(i18n.translate("popup_dialog.ok_button")))
      .on_press(Message::OnOkButtonPress(self.id));

    Column::new()
    .push(title)
    .push(content)
    .push(ok_button)
    .into()
  }

}
