use std::sync::{Arc, Mutex};

use iced::{widget::{scrollable, Button, Column, Container, Row, Scrollable, Space, Text}, Alignment, Element, Length};
use log::warn;

use crate::{database::{account::Account, Database}, font_icon, i18n::I18n, style_variable::StyleVariable, util::account_util, widget::common};

const MODULE_PATH: &str = module_path!();

pub struct AccountDetailDialog {

  account_id: usize,

  censor_password: bool,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnCloseButtonPress,

  OnCensorSwitchPress,

}

impl AccountDetailDialog {

  pub fn new(account_id: usize) -> Self {
    Self {
      account_id,
      censor_password: true,
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::OnCloseButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnCloseButtonPress should be intercepted");
      }
      Message::OnCensorSwitchPress => {
        self.censor_password = !self.censor_password;
      }
    }
  }

  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let content = self.view_content(
      i18n,
      database.accounts().get(self.account_id)
        .expect(&format!("Account id ({}) out of bounds", self.account_id))
        .as_ref()
        .expect(&format!("Account (id={}) is deleted", self.account_id)),
      database,
      style_variable,
    );

    let style_variable = StyleVariable::lock(style_variable);

    Row::new()
    .push(
      Space::new(Length::FillPortion(1), Length::Fill)
    )
    .push(
      Column::new()
      .push(
        Space::new(Length::Fill, Length::Fixed(64_f32))
      )
      .push(
        Column::new()
        .push(
          Row::new()
          .push(
            Text::new(i18n.translate("account_detail_dialog.title"))
          )
          .push(
            Space::new(Length::Fill, Length::Shrink)
          )
          .push(
            Button::new(Text::new("X"))
            .on_press(Message::OnCloseButtonPress)
          )
        )
        .push(
          Scrollable::new(
            content
            .padding(style_variable.account_detail_dialog_content_padding)
          )
          .width(Length::Fill)
          .height(Length::Fill)
          .direction(
            scrollable::Direction::Vertical(
              scrollable::Scrollbar::new()
              .width(style_variable.account_detail_dialog_scrollbar_width)
              .margin(style_variable.account_detail_dialog_scrollbar_margin)
              .scroller_width(style_variable.account_detail_dialog_scroller_width)
              .anchor(scrollable::Anchor::Start)
            )
          )
        )
        .width(Length::Fill)
        .height(Length::Fill)
      )
      .push(
        Space::new(Length::Fill, Length::Fixed(64_f32))
      )
      .width(Length::FillPortion(2))
      .height(Length::Fill)
    )
    .push(
      Space::new(Length::FillPortion(1), Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }

  fn view_content(&self, i18n: &I18n, account: &Account, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    Column::new()
    .push(
      Container::new(
        Text::new(account.name().to_string())
      )
      .width(Length::Fill)
      .height(Length::Shrink)
      .align_x(Alignment::Center)
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.parent_account"))
      )
      .push(
        Text::new(
          if let Some(parent_account) = account.parent_account() {
            account_util::get_account_short_form(parent_account, database)
          }
          else {
            "None".to_string()
          }
        )
      )
    )
    .push(
      Text::new(i18n.translate("account_detail_dialog.children_accounts"))
    )
    .push(
      self.view_children_accounts(account, database, style_variable)
    )
    .push(
      Text::new(i18n.translate("account_detail_dialog.reference_accounts"))
    )
    .push(
      self.view_reference_accounts(account, database, style_variable)
    )
    .push(
      Text::new(i18n.translate("account_detail_dialog.referenced_by_accounts"))
    )
    .push(
      self.view_referenced_by_accounts(account, database, style_variable)
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.service"))
      )
      .push(
        Text::new(
          if let Some(service) = account.service() {
            service.clone()
          }
          else {
            "".to_string()
          }
        )
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.login_name"))
      )
      .push(
        Text::new(
          if let Some(login_name) = account.login_name() {
            login_name.clone()
          }
          else {
            "".to_string()
          }
        )
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.password"))
      )
      .push(
        if let Some(password) = account.password() {
          if self.censor_password {
            font_icon::stop_circle_round_x6()
          }
          else {
            Text::new(password.clone())
          }
        }
        else {
          Text::new("")
        }
      )
      .push(
        common::create_censor_switch_button(self.censor_password, style_variable)
        .on_press(Message::OnCensorSwitchPress)
      )
    )
    .push(
      Text::new(i18n.translate("account_detail_dialog.comment"))
    )
    .push(
      Text::new(
        if let Some(comment) = account.comment() {
          comment.clone()
        }
        else {
          "".to_string()
        }
      )
    )
    .push(
      Text::new(i18n.translate("account_detail_dialog.custom_fields"))
    )
    .push(
      self.view_custom_fields(account, style_variable)
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.create_time"))
      )
      .push(
        Text::new(account.create_time().format("%Y/%m/%d %H:%M:%S").to_string())
      )
    )
    .push(
      Row::new()
      .push(
        Text::new(i18n.translate("account_detail_dialog.modify_time"))
      )
      .push(
        Text::new(account.modify_time().format("%Y/%m/%d %H:%M:%S").to_string())
      )
    )
    .width(Length::Fill)
  }

  fn view_children_accounts(&self, account: &Account, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    let mut column = Column::new();
    for child_account in account.children_accounts() {
      column = column.push(
        Text::new(account_util::get_account_short_form(*child_account, database))
      );
    }
    column
  }

  fn view_reference_accounts(&self, account: &Account, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    let mut column = Column::new();
    for reference_account in account.reference_accounts() {
      column = column.push(
        Text::new(account_util::get_account_short_form(*reference_account, database))
      );
    }
    column
  }

  fn view_referenced_by_accounts(&self, account: &Account, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    let mut column = Column::new();
    for referenced_by_account in account.referenced_by_accounts() {
      column = column.push(
        Text::new(account_util::get_account_short_form(*referenced_by_account, database))
      );
    }
    column
  }

  fn view_custom_fields(&self, account: &Account, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    let mut column = Column::new();
    for custom_field in account.custom_fields() {
      column = column.push(
        Text::new(format!("{}: {}", custom_field.0, custom_field.1))
      );
    }
    column
  }

}
