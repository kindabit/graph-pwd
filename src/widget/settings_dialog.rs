use std::sync::{Arc, Mutex};

use iced::{widget::{Button, Column, Row, Slider, Text}, Element};
use log::warn;

use crate::{config::Config, i18n::I18n, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

const CLEAR_CLIPBOARD_COUNTDOWN_MIN: i32 = 5;
const CLEAR_CLIPBOARD_COUNTDOWN_MAX: i32 = 60;

pub struct SettingsDialog {

  clear_clipboard_countdown: i32,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnConfirmButtonPress,

  OnCancelButtonPress,

  OnClearClipboardCountdownChange(i32),

}

impl SettingsDialog {

  pub fn new(config: &Config) -> Self {
    Self {
      clear_clipboard_countdown: config.clear_clipboard_countdown(),
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::OnConfirmButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnConfirmButtonPress should be intercepted");
      }
      Message::OnCancelButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnCancelButtonPress should be intercepted");
      }
      Message::OnClearClipboardCountdownChange(value) => {
        self.clear_clipboard_countdown = value;
      }
    }
  }

  pub fn view(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    Column::new()
    .push(
      Text::new(i18n.translate("settings_dialog.title"))
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("settings_dialog.clear_clipboard_countdown_title"))
      )
      .push(
        Slider::new(
          CLEAR_CLIPBOARD_COUNTDOWN_MIN..=CLEAR_CLIPBOARD_COUNTDOWN_MAX,
          self.clear_clipboard_countdown,
          Message::OnClearClipboardCountdownChange
        )
      )
      .push(
        Text::new(self.clear_clipboard_countdown.to_string())
      )
      .push(
        Text::new(i18n.translate("settings_dialog.clear_clipboard_countdown_unit"))
      )
    )
    .push(
      Row::new()
      .push(
        Button::new(Text::new(i18n.translate("settings_dialog.confirm")))
        .on_press(Message::OnConfirmButtonPress)
      )
      .push(
        Button::new(Text::new(i18n.translate("settings_dialog.cancel")))
        .on_press(Message::OnCancelButtonPress)
      )
    )
    .into()
  }

  pub fn clear_clipboard_countdown(&self) -> i32 {
    self.clear_clipboard_countdown
  }

}
