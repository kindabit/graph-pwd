use iced::{widget::{Button, Column, Row, Text}, Element};

use crate::{font_icon, i18n::I18n};

pub struct HelpDialog {
}

#[derive(Clone, Debug)]
pub enum Message {

  OnCloseButtonPress,

}

impl HelpDialog {

  pub fn new() -> Self {
    Self {
    }
  }

  pub fn view(&self, i18n: &I18n) -> Element<Message> {
    Column::new()
    .push(
      Text::new(i18n.translate("help_dialog.title"))
    )
    .push(
      Text::new(i18n.translate("help_dialog.shortcut_title"))
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("help_dialog.shortcut_save_database_title"))
      )
      .push(
        Text::new(i18n.translate("help_dialog.shortcut_save_database_content"))
      )
    )
    .push(
      Row::new()
      .push(
        font_icon::mouse_round()
      )
      .push(
        Text::new(i18n.translate("help_dialog.shortcut_click_to_copy_content"))
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("help_dialog.terminology_title"))
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("help_dialog.terminology_account_name_title"))
      )
      .push(
        Text::new(i18n.translate("help_dialog.terminology_account_name_content"))
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("help_dialog.terminology_account_login_name_title"))
      )
      .push(
        Text::new(i18n.translate("help_dialog.terminology_account_login_name_content"))
      )
    )
    .push(
      Button::new(
        Text::new(i18n.translate("help_dialog.close"))
      )
      .on_press(Message::OnCloseButtonPress)
    )
    .into()
  }

}
