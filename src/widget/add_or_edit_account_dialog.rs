use std::{collections::{BTreeMap, BTreeSet, HashSet}, sync::{Arc, Mutex}};

use iced::{widget::{scrollable, Button, Column, Row, Scrollable, Space, Text, TextInput}, Alignment, Element, Length};
use log::warn;

use crate::{database::{account::Account, Database}, i18n::I18n, style_variable::StyleVariable, util::{account_util, tree_util}};

use super::MiniAccountSelector;

const MODULE_PATH: &str = module_path!();

pub enum AddOrEditAccountDialogMode {

  Add,

  Edit,

}

enum NameError {

  Empty,

}

pub struct AddOrEditAccountDialog {

  mode: AddOrEditAccountDialogMode,

  id: Option<usize>,

  account_selector: MiniAccountSelector,

  /// Option<id>
  parent_account: Option<usize>,

  parent_account_search: String,

  /// BTreeSet<id>
  reference_accounts: BTreeSet<usize>,

  reference_accounts_search: String,

  name: String,

  name_error: Option<NameError>,

  service: Option<String>,

  login_name: Option<String>,

  password: Option<String>,

  comment: Option<String>,

  custom_fields: BTreeMap<String, String>,

  custom_field_name: String,

  custom_field_value: String,

}

#[derive(Clone, Debug)]
pub enum Message {

  OnClearParentAccountPress,

  OnParentAccountSearchInputInput(String),

  ParentAccountSelectorMessage(super::MiniAccountSelectorMessage),

  OnClearReferenceAccountPress(usize),

  OnReferenceAccountsSearchInputInput(String),

  ReferenceAccountsSelectorMessage(super::MiniAccountSelectorMessage),

  OnNameInputInput(String),

  OnServiceInputInput(String),

  OnLoginNameInputInput(String),

  OnPasswordInputInput(String),

  OnCommentInputInput(String),

  OnRemoveCustomFieldPress(String),

  OnCustomFieldNameInputInput(String),

  OnCustomFieldValueInputInput(String),

  OnAddCustomFieldPress,

  OnConfirmButtonPress,

  OnCancelButtonPress,

}

impl AddOrEditAccountDialog {

  pub fn new(mode: AddOrEditAccountDialogMode, old_account: Option<&Account>) -> Self {
    match mode {
      AddOrEditAccountDialogMode::Add => {
        match old_account {
          Some(_) => {
            panic!("when mode is AddOrEditAccountDialogMode::Add, old_account must be None");
          }
          None => {
            Self {
              mode,
              id: None,
              account_selector: MiniAccountSelector::new(),
              parent_account: None,
              parent_account_search: String::new(),
              reference_accounts: BTreeSet::new(),
              reference_accounts_search: String::new(),
              name: String::new(),
              name_error: Some(NameError::Empty),
              service: None,
              login_name: None,
              password: None,
              comment: None,
              custom_fields: BTreeMap::new(),
              custom_field_name: String::new(),
              custom_field_value: String::new(),
            }
          }
        }
      }
      AddOrEditAccountDialogMode::Edit => {
        match old_account {
          Some(old_account) => {
            Self {
              mode,
              id: Some(old_account.id()),
              account_selector: MiniAccountSelector::new(),
              parent_account: old_account.parent_account(),
              parent_account_search: String::new(),
              reference_accounts: old_account.reference_accounts().clone(),
              reference_accounts_search: String::new(),
              name: old_account.name().to_string(),
              name_error: None,
              service: old_account.service().map(String::from),
              login_name: old_account.login_name().map(String::from),
              password: old_account.password().map(String::from),
              comment: old_account.comment().map(String::from),
              custom_fields: {
                let mut custom_fields: BTreeMap<String, String> = BTreeMap::new();
                for custom_field in old_account.custom_fields() {
                  custom_fields.insert(custom_field.0.to_string(), custom_field.1.to_string());
                }
                custom_fields
              },
              custom_field_name: String::new(),
              custom_field_value: String::new(),
            }
          }
          None => {
            panic!("when mode is AddOrEditAccountDialogMode::Edit, old_account must be Some");
          }
        }
      }
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::OnClearParentAccountPress => {
        self.parent_account = None;
      }
      Message::OnParentAccountSearchInputInput(value) => {
        self.parent_account_search = value;
      }
      Message::ParentAccountSelectorMessage(msg) => {
        match msg {
          super::MiniAccountSelectorMessage::OnRowPress(id) => {
            self.parent_account = Some(id);
          }
        }
      }
      Message::OnClearReferenceAccountPress(id) => {
        self.reference_accounts.remove(&id);
      }
      Message::OnReferenceAccountsSearchInputInput(value) => {
        self.reference_accounts_search = value;
      }
      Message::ReferenceAccountsSelectorMessage(msg) => {
        match msg {
          super::MiniAccountSelectorMessage::OnRowPress(id) => {
            self.reference_accounts.insert(id);
          }
        }
      }
      Message::OnNameInputInput(value) => {
        self.name = value;
        if self.name.len() == 0 {
          self.name_error = Some(NameError::Empty);
        }
        else {
          self.name_error = None;
        }
      }
      Message::OnServiceInputInput(value) => {
        if value.len() == 0 {
          self.service = None;
        }
        else {
          self.service = Some(value)
        }
      }
      Message::OnLoginNameInputInput(value) => {
        if value.len() == 0 {
          self.login_name = None;
        }
        else {
          self.login_name = Some(value)
        }
      }
      Message::OnPasswordInputInput(value) => {
        if value.len() == 0 {
          self.password = None;
        }
        else {
          self.password = Some(value)
        }
      }
      Message::OnCommentInputInput(value) => {
        if value.len() == 0 {
          self.comment = None;
        }
        else {
          self.comment = Some(value)
        }
      }
      Message::OnRemoveCustomFieldPress(field_name) => {
        self.custom_fields.remove(&field_name);
      }
      Message::OnCustomFieldNameInputInput(value) => {
        self.custom_field_name = value;
      }
      Message::OnCustomFieldValueInputInput(value) => {
        self.custom_field_value = value;
      }
      Message::OnAddCustomFieldPress => {
        let custom_field_name = self.custom_field_name.trim();
        if custom_field_name.len() > 0 {
          self.custom_fields.insert(custom_field_name.to_string(), self.custom_field_value.clone());
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

  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let form = self.view_form(i18n, database, style_variable);

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
          match self.mode {
            AddOrEditAccountDialogMode::Add => Text::new(i18n.translate("add_or_edit_account_dialog.title_add")),
            AddOrEditAccountDialogMode::Edit => Text::new(i18n.translate("add_or_edit_account_dialog.title_edit")),
          }
          .width(Length::Fill)
          .height(Length::Shrink)
        )
        .push(
          Scrollable::new(
            form
            .padding(style_variable.add_or_edit_account_dialog_form_padding)
          )
          .width(Length::Fill)
          .height(Length::Fill)
          .direction(
            scrollable::Direction::Vertical(
              scrollable::Scrollbar::new()
              .width(style_variable.add_or_edit_account_dialog_scrollbar_width)
              .margin(style_variable.add_or_edit_account_dialog_scrollbar_margin)
              .scroller_width(style_variable.add_or_edit_account_dialog_scroller_width)
              .anchor(scrollable::Anchor::Start)
            )
          )
        )
        .push(
          Row::new()
          .push(
            Button::new(Text::new(i18n.translate("add_or_edit_account_dialog.confirm_button")))
            .on_press(Message::OnConfirmButtonPress)
          )
          .push(
            Button::new(Text::new(i18n.translate("add_or_edit_account_dialog.cancel_button")))
            .on_press(Message::OnCancelButtonPress)
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

  fn view_form(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Column<Message> {
    let mut form = Column::new();

    // parent account - selected parent account row

    let mut selected_parent_account_row = Row::new();

    let parent_account_label = Text::new(i18n.translate("add_or_edit_account_dialog.parent_account"));
    selected_parent_account_row = selected_parent_account_row.push(parent_account_label);

    if let Some(parent_account) = self.parent_account.as_ref() {
      selected_parent_account_row = selected_parent_account_row.push(Text::new(account_util::get_account_short_form(*parent_account, database)));

      let clear_parent_account = Button::new(Text::new("X")).on_press(Message::OnClearParentAccountPress);
      selected_parent_account_row = selected_parent_account_row.push(clear_parent_account);
    }

    form = form.push(selected_parent_account_row.align_y(Alignment::Center));

    // parent account - parent account search

    form = form.push(
      TextInput::new(&i18n.translate("add_or_edit_account_dialog.parent_account_search_placeholder"), &self.parent_account_search)
      .on_input(Message::OnParentAccountSearchInputInput)
    );

    // parent account - parent account selector

    // self, children and children's children are all excluded, to avoid "paradox"
    let mut exclude_account_ids = HashSet::new();
    if let Some(self_id) = self.id {
      tree_util::traverse_account_tree(self_id, database, &mut |account| {
        exclude_account_ids.insert(account.id());
      });
    }

    let parent_account_selector = match self.parent_account.as_ref() {
      Some(parent_account) => {
        self.account_selector.view(
          database,
          &self.parent_account_search,
          &exclude_account_ids,
          &[*parent_account],
          style_variable,
        )
      }
      None => {
        self.account_selector.view(
          database,
          &self.parent_account_search,
          &exclude_account_ids,
          &[],
          style_variable,
        )
      }
    };

    form = form.push(parent_account_selector.map(Message::ParentAccountSelectorMessage));

    // reference accounts - reference accounts title row

    let mut reference_accounts_title_row = Row::new();

    let reference_accounts_title = Text::new(i18n.translate("add_or_edit_account_dialog.reference_accounts"));
    reference_accounts_title_row = reference_accounts_title_row.push(reference_accounts_title);

    form = form.push(reference_accounts_title_row);

    // reference accounts - selected reference accounts

    for reference_account in &self.reference_accounts {
      form = form.push(
        Row::new()
        .push(
          Text::new(account_util::get_account_short_form(*reference_account, database))
        )
        .push(
          Button::new(Text::new("X")).on_press(Message::OnClearReferenceAccountPress(*reference_account))
        )
        .align_y(Alignment::Center)
      )
    }

    // reference accounts - reference accounts search

    form = form.push(
      TextInput::new(&i18n.translate("add_or_edit_account_dialog.reference_accounts_search_placeholder"), &self.reference_accounts_search)
      .on_input(Message::OnReferenceAccountsSearchInputInput)
    );

    // reference accounts - reference accounts selector

    // self referencing is not allowed
    let mut exclude_account_ids = HashSet::new();
    if let Some(self_id) = self.id {
      exclude_account_ids.insert(self_id);
    }

    let reference_account_ids: Vec<usize> = self.reference_accounts.iter().map(|pair| *pair).collect();
    let reference_accounts_selector = self.account_selector.view(
      database,
      &self.reference_accounts_search,
      &exclude_account_ids,
      &reference_account_ids,
      style_variable,
    );

    form = form.push(reference_accounts_selector.map(Message::ReferenceAccountsSelectorMessage));

    // name

    form = form.push(
      Row::new()
      .push(
        Text::new(i18n.translate("add_or_edit_account_dialog.name"))
      )
      .push(
        TextInput::new(&i18n.translate("add_or_edit_account_dialog.name_placeholder"), &self.name)
        .on_input(Message::OnNameInputInput)
      )
      .align_y(Alignment::Center)
    );

    if let Some(name_error) = self.name_error.as_ref() {
      match name_error {
        NameError::Empty => {
          form = form.push(
            Text::new(i18n.translate(&i18n.translate("add_or_edit_account_dialog.name_error_empty")))
          )
        }
      }
    }

    // service

    form = form.push(
      Row::new()
      .push(
        Text::new(i18n.translate("add_or_edit_account_dialog.service"))
      )
      .push(
        TextInput::new(
          &i18n.translate("add_or_edit_account_dialog.service_placeholder"),
          match self.service.as_ref() {
            Some(service) => {
              service
            }
            None => {
              ""
            }
          }
        )
        .on_input(Message::OnServiceInputInput)
      )
      .align_y(Alignment::Center)
    );

    // login name

    form = form.push(
      Row::new()
      .push(
        Text::new(i18n.translate("add_or_edit_account_dialog.login_name"))
      )
      .push(
        TextInput::new(
          &i18n.translate("add_or_edit_account_dialog.login_name_placeholder"),
          match self.login_name.as_ref() {
            Some(login_name) => {
              login_name
            }
            None => {
              ""
            }
          }
        )
        .on_input(Message::OnLoginNameInputInput)
      )
      .align_y(Alignment::Center)
    );

    // password

    form = form.push(
      Row::new()
      .push(
        Text::new(i18n.translate("add_or_edit_account_dialog.password"))
      )
      .push(
        TextInput::new(
          &i18n.translate("add_or_edit_account_dialog.password_placeholder"),
          match self.password.as_ref() {
            Some(password) => {
              password
            }
            None => {
              ""
            }
          }
        )
        .secure(true)
        .on_input(Message::OnPasswordInputInput)
      )
      .align_y(Alignment::Center)
    );

    // comment

    form = form.push(
      Row::new()
      .push(
        Text::new(i18n.translate("add_or_edit_account_dialog.comment"))
      )
      .push(
        TextInput::new(
          &i18n.translate("add_or_edit_account_dialog.comment_placeholder"),
          match self.comment.as_ref() {
            Some(comment) => {
              comment
            }
            None => {
              ""
            }
          }
        )
        .on_input(Message::OnCommentInputInput)
      )
      .align_y(Alignment::Center)
    );

    // custom fields - title

    form = form.push(
      Text::new(i18n.translate("add_or_edit_account_dialog.custom_field"))
    );

    // custom field - current custom field

    for custom_field in self.custom_fields.iter() {
      form = form.push(
        Row::new()
        .push(
          Text::new(format!("{}: {}", custom_field.0, custom_field.1))
        )
        .push(
          Button::new(Text::new("X")).on_press(Message::OnRemoveCustomFieldPress(custom_field.0.to_string()))
        )
      );
    }

    // custom field - add custom field

    form = form.push(
      Row::new()
      .push(
        TextInput::new(&i18n.translate("add_or_edit_account_dialog.custom_field_name_placeholder"), &self.custom_field_name)
        .on_input(Message::OnCustomFieldNameInputInput)
      )
      .push(
        Text::new(":")
      )
      .push(
        TextInput::new(&i18n.translate("add_or_edit_account_dialog.custom_field_value_placeholder"), &self.custom_field_value)
        .on_input(Message::OnCustomFieldValueInputInput)
      )
      .push(
        Button::new(Text::new(i18n.translate("add_or_edit_account_dialog.add_custom_field")))
        .on_press(Message::OnAddCustomFieldPress)
      )
      .align_y(Alignment::Center)
    );

    form
  }

  /// use `&mut self` because need to trim string before validating
  pub fn validate(&mut self) -> bool {
    self.trim_fields();

    if let Some(self_id) = self.id {
      if let Some(parent_id) = self.parent_account && self_id == parent_id {
        panic!("Self referencing detected in parent account");
      }
      if self.reference_accounts.contains(&self_id) {
        panic!("Self referencing detected in reference accounts");
      }
    }

    if self.name_error.is_some() {
      false
    }
    else {
      true
    }
  }

  fn trim_fields(&mut self) {
    self.name = self.name.trim().to_string();
    if self.name.len() == 0 {
      self.name_error = Some(NameError::Empty);
    }
    else {
      self.name_error = None;
    }
  }

}

impl AddOrEditAccountDialog {

  pub fn mode(&self) -> &AddOrEditAccountDialogMode {
    &self.mode
  }

  pub fn id(&self) -> Option<usize> {
    self.id
  }

  pub fn parent_account(&self) -> Option<usize> {
    self.parent_account
  }

  pub fn reference_accounts(&self) -> &BTreeSet<usize> {
    &self.reference_accounts
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn service(&self) -> Option<&String> {
    self.service.as_ref()
  }

  pub fn login_name(&self) -> Option<&String> {
    self.login_name.as_ref()
  }

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }

  pub fn comment(&self) -> Option<&String> {
    self.comment.as_ref()
  }

  pub fn custom_fields(&self) -> &BTreeMap<String, String> {
    &self.custom_fields
  }

}
