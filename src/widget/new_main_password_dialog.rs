use iced::{widget::{Button, Column, Row, Text, TextInput}, Element};
use log::warn;

use crate::{i18n::I18n, util::account_util};

const MODULE_PATH: &str = module_path!();

enum PasswordError {

  TooWeak,

  Empty,

}

enum RepeatPasswordError {

  NotIdentical,

}

pub struct NewMainPasswordDialog {

  password: String,

  password_error: Option<PasswordError>,

  repeat_password: String,

  repeat_password_error: Option<RepeatPasswordError>,

  on_confirm_message: crate::Message,

  on_cancel_message: crate::Message,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnPasswordInputInput(String),

  OnRepeatPasswordInputInput(String),

  OnConfirmButtonPress,

  OnCancelButtonPress,

}

impl NewMainPasswordDialog {

  pub fn new(on_confirm_message: crate::Message, on_cancel_message: crate::Message) -> Self {
    Self {
      password: String::new(),
      password_error: Some(PasswordError::Empty),
      repeat_password: String::new(),
      repeat_password_error: None,
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
        else if account_util::is_weak_password(&self.password) {
          self.password_error = Some(PasswordError::TooWeak);
        }
        else {
          self.password_error = None;
        }
        if self.password != self.repeat_password {
          self.repeat_password_error = Some(RepeatPasswordError::NotIdentical);
        }
        else {
          self.repeat_password_error = None;
        }
      }
      Message::OnRepeatPasswordInputInput(repeat_password) => {
        self.repeat_password = repeat_password;
        if self.password != self.repeat_password {
          self.repeat_password_error = Some(RepeatPasswordError::NotIdentical);
        }
        else {
          self.repeat_password_error = None;
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
      Text::new(i18n.translate("new_main_password_dialog.title"))
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("new_main_password_dialog.password"))
      )
      .push(
        TextInput::new(&i18n.translate("new_main_password_dialog.password_placeholder"), &self.password)
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
              PasswordError::Empty => i18n.translate("new_main_password_dialog.empty_password"),
              PasswordError::TooWeak => i18n.translate("new_main_password_dialog.weak_password"),
            }
          },
          None => "".to_string(),
        }
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("new_main_password_dialog.repeat_password"))
      )
      .push(
        TextInput::new(&i18n.translate("new_main_password_dialog.repeat_password_placeholder"), &self.repeat_password)
        .secure(true)
        .on_input(Message::OnRepeatPasswordInputInput)
        .on_paste(Message::OnRepeatPasswordInputInput)
      )
    )
    .push(
      Text::new(
        match self.repeat_password_error.as_ref() {
          Some(err) => {
            match err {
              RepeatPasswordError::NotIdentical => i18n.translate("new_main_password_dialog.not_identical"),
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
          Text::new(i18n.translate("new_main_password_dialog.confirm"))
        )
        .on_press(Message::OnConfirmButtonPress)
      )
      .push(
        Button::new(
          Text::new(i18n.translate("new_main_password_dialog.cancel"))
        )
        .on_press(Message::OnCancelButtonPress)
      )
    )
    .into()
  }

  pub fn validate(&self) -> bool {
    if let Some(err) = &self.password_error {
      match err {
        PasswordError::TooWeak => {},
        PasswordError::Empty => return false,
      }
    }
    if let Some(err) = &self.repeat_password_error {
      match err {
        RepeatPasswordError::NotIdentical => return false,
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
