use std::sync::{Arc, Mutex};

use iced::{widget::{Button, Column, Text}, Element};

use crate::{database::Database, i18n::I18n, style_variable::StyleVariable, util::account_util};

pub struct AccountListDialog {

  title: String,

  account_ids: Vec<usize>,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnCloseButtonPress,

}

impl AccountListDialog {

  pub fn new(title: String, account_ids: Vec<usize>) -> Self {
    Self {
      title,
      account_ids,
    }
  }

  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut column = Column::new()
    .push(
      Text::new(&self.title)
    );

    if self.account_ids.len() > 0 {
      for account_id in &self.account_ids {
        column = column.push(
          Text::new(account_util::get_account_short_form(*account_id, database))
        );
      }
    }
    else {
      column = column.push(
        Text::new(i18n.translate("account_list_dialog.empty"))
      );
    }

    column.push(
      Button::new(
        Text::new(i18n.translate("account_list_dialog.close"))
      )
      .on_press(Message::OnCloseButtonPress)
    )
    .into()
  }

}
