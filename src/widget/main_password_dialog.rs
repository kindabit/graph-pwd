use iced::{widget::{Button, Column, Row, Text, TextInput}, Element};
use log::warn;

use crate::i18n::I18n;

const MODULE_PATH: &str = module_path!();

enum PasswordError {

  Empty,

}

pub struct MainPasswordDialog {

  password: String,

  password_error: Option<PasswordError>,

  on_confirm_message: crate::Message,

  on_cancel_message: crate::Message,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnPasswordInputInput(String),

  OnConfirmButtonPress,

  OnCancelButtonPress,

}

impl MainPasswordDialog {

  pub fn new(on_confirm_message: crate::Message, on_cancel_message: crate::Message) -> Self {
    Self {
      password: String::new(),
      password_error: Some(PasswordError::Empty),
      on_confirm_message,
      on_cancel_message,
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::OnPasswordInputInput(password) => {
        self.password = password;
        if self.password.is_empty() {
          self.password_error = Some(PasswordError::Empty);
        }
        else {
          self.password_error = None;
        }
      }
      Message::OnConfirmButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnConfirmButtonPress should be intercepted");
      }
      Message::OnCancelButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnCancelButtonPress should be intercepted");
      }
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    Column::new()
    .push(
      Text::new(i18n.translate("main_password_dialog.title"))
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("main_password_dialog.password"))
      )
      .push(
        TextInput::new(&i18n.translate("main_password_dialog.password_placeholder"), &self.password)
        .secure(true)
        .on_input(Message::OnPasswordInputInput)
        .on_paste(Message::OnPasswordInputInput)
      )
    )
    .push(
      Text::new(
        match self.password_error.as_ref() {
          Some(err) => {
            match err {
              PasswordError::Empty => i18n.translate("main_password_dialog.empty_password"),
            }
          },
          None => "".to_string(),
        }
      )
    )
    .push(
      Row::new()
      .push(
        Button::new(
          Text::new(i18n.translate("main_password_dialog.confirm"))
        )
        .on_press(Message::OnConfirmButtonPress)
      )
      .push(
        Button::new(
          Text::new(i18n.translate("main_password_dialog.cancel"))
        )
        .on_press(Message::OnCancelButtonPress)
      )
    )
    .into()
  }

  pub fn validate(&self) -> bool {
    if let Some(err) = &self.password_error {
      match err {
        PasswordError::Empty => return false,
      }
    }
    true
  }

  pub fn into_on_confirm_message(self) -> (crate::Message, String) {
    (self.on_confirm_message, self.password)
  }

  pub fn into_on_cancel_message(self) -> crate::Message {
    self.on_cancel_message
  }

}
