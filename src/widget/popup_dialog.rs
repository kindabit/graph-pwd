use iced::{widget::{button, column, text}, Element};

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

  OnOkButtonClicked(usize),

}

impl PopupDialog {

  pub fn new(id: usize, title: String, content: String, r#type: PopupDialogType) -> Self {
    Self {
      id,
      title,
      content,
      r#type,
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    let title = text(i18n.translate(&self.title));

    let content = text(&self.content);

    let ok_button = button(text(i18n.translate("popup_dialog.ok_button")))
      .on_press(Message::OnOkButtonClicked(self.id));

    column![
      title,
      content,
      ok_button,
    ].into()
  }

}
