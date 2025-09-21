#![allow(unused_braces)]

mod util;
mod app_error;
mod logging;
mod config;
mod i18n;
mod font_icon;
mod style_variable;
mod database;
mod widget;

use std::{cell::RefCell, env, error::Error, process::{Command, ExitCode}, rc::Rc, sync::{Arc, Mutex}};

use config::Config;
use i18n::I18n;
use iced::{application::Boot, keyboard, wgpu::core::id, widget::column, window::Position, Element, Font, Length, Task};
use log::{debug, info};
use logging::setup_logging;

use crate::{database::{account::Account, Database}, i18n::Language, style_variable::StyleVariable, util::{account_util, modal}};

const EXIT_CODE_RESTART: u8 = 2;

struct RootWidgetBoot {

  exit_code: Rc<RefCell<u8>>,

}

impl RootWidgetBoot {

  pub fn new(exit_code: Rc<RefCell<u8>>) -> Self {
    Self {
      exit_code,
    }
  }

}

impl Boot<RootWidget, Message> for RootWidgetBoot {

  fn boot(&self) -> (RootWidget, Task<Message>) {
    let config = Config::new().expect("fail to initialize config");
    debug!("config: {config:?}");

    let i18n = I18n::new(&config).expect("fail to initialize i18n");
    debug!("i18n: {i18n:?}");

    (
      RootWidget::new(self.exit_code.clone(), config, i18n),
      Task::none(),
    )
  }

}

pub fn main() -> Result<ExitCode, Box<dyn Error>> {
  if env::args().any(|arg| arg.eq("client")) {
    let exit_code = Rc::new(RefCell::new(0_u8));

    setup_logging(false)?;

    let app = iced::application(
      RootWidgetBoot::new(exit_code.clone()),
      RootWidget::update,
      RootWidget::view
    )
    .font(include_bytes!("./assets/思源黑体.otf"))
    .default_font(Font::with_name("思源黑体"));

    font_icon::load(app)
    .title("Graph PWD")
    .window(iced::window::Settings {
      position: Position::Specific([0_f32, 0_f32].into()),
      size: [800_f32, 600_f32].into(),
      maximized: true,
      ..Default::default()
    })
    .subscription(|_state| {
      keyboard::on_key_press(|key, modifiers| {
        match key {
          keyboard::Key::Named(_named) => {
            None
          }
          keyboard::Key::Character(ch) => {
            if modifiers.control() && ch.eq_ignore_ascii_case("s") {
              Some(Message::SaveDatabase)
            }
            else {
              None
            }
          },
          keyboard::Key::Unidentified => {
            None
          },
        }
      })
    })
    .run()?;

    let exit_code = *exit_code.borrow();

    info!("Exiting with exit code {exit_code}");

    Ok(ExitCode::from(exit_code))
  }
  else {
    setup_logging(true)?;

    let exit_code = loop {
      let exe_path = env::current_exe().expect("Fail to get path of current executable file");

      info!("Launching program {exe_path:?}");

      let mut child = Command::new(exe_path.as_os_str()).arg("client").spawn()?;

      info!("Waiting for child to exit");

      let exit_status = child.wait()?;

      info!("Child exited with status {exit_status:?}");

      if let Some(code) = exit_status.code() {
        if code == EXIT_CODE_RESTART as i32 {
          info!("Exit code {code} indicates a restart is needed");
        }
        else {
          break code as u8
        }
      }
      else {
        break 0_u8
      }
    };

    info!("Exiting daemon process with exit code {exit_code}");

    Ok(ExitCode::from(exit_code))
  }
}

#[derive(Clone, Debug)]
pub enum DatabaseUpdatedType {
  NewDatabase,
  NewRootAccount,
  NewChildAccount(usize),
  ModifyAccount(usize),
  DeleteAccount(usize),
}

#[derive(Clone, Debug)]
pub enum Message {

  HeaderMessage(widget::HeaderMessage),
  WorkingAreaMessage(widget::WorkingAreaMessage),
  StatusBarMessage(widget::StatusBarMessage),
  PopupDialogMessage(widget::PopupDialogMessage),
  ConfirmDialogMessage(widget::ConfirmDialogMessage),
  AddOrEditAccountDialogMessage(widget::AddOrEditAccountDialogMessage),
  AccountDetailDialogMessage(widget::AccountDetailDialogMessage),
  AccountListDialogMessage(widget::AccountListDialogMessage),
  NewMainPasswordDialogMessage(widget::NewMainPasswordDialogMessage),
  MainPasswordDialogMessage(widget::MainPasswordDialogMessage),
  HelpDialogMessage(widget::HelpDialogMessage),

  NewDatabase,
  NewDatabaseConfirmed,
  NewDatabaseSelected(Option<String>),
  NewDatabaseMainPasswordInputted(String),
  NewDatabaseSuccess,
  NewDatabaseFail(String),

  LoadDatabase,
  LoadDatabaseConfirmed,
  LoadDatabaseSelected(Option<String>),
  LoadDatabaseMainPasswordInputted(String),
  LoadDatabaseSuccess,
  LoadDatabaseFail(String),

  SaveDatabase,

  SaveAsDatabase,
  SaveAsDatabaseSelected(Option<String>),
  SaveAsDatabaseSuccess,
  SaveAsDatabaseFail(String),

  DeleteAccountConfirmed(usize),

  ChangeLanguageConfirmed(Language),

  Noop,

}

struct RootWidget {

  config: Config,

  i18n: I18n,

  style_variable: Arc<Mutex<StyleVariable>>,

  database: Rc<RefCell<Option<Database>>>,

  popup_dialogs: Vec<widget::PopupDialog>,

  confirm_dialogs: Vec<widget::ConfirmDialog>,

  add_or_edit_account_dialog: Option<widget::AddOrEditAccountDialog>,

  account_detail_dialog: Option<widget::AccountDetailDialog>,

  account_list_dialog: Option<widget::AccountListDialog>,

  new_main_password_dialog: Option<widget::NewMainPasswordDialog>,

  main_password_dialog: Option<widget::MainPasswordDialog>,

  help_dialog: Option<widget::HelpDialog>,

  header: widget::Header,

  working_area: widget::WorkingArea,

  status_bar: widget::StatusBar,

  // todo: this is awkward
  temp_password: String,

  exit_code: Rc<RefCell<u8>>,

}

impl RootWidget {

  pub fn new(exit_code: Rc<RefCell<u8>>, config: Config, i18n: I18n) -> Self {
    let database = Rc::new(RefCell::new(None));

    let initial_tree_mode = config.tree_mode();
    let available_languages = i18n.available_languages().to_vec();

    Self {
      config,

      i18n,

      style_variable: Arc::new(Mutex::new(StyleVariable::new())),

      database: database.clone(),

      popup_dialogs: Vec::new(),

      confirm_dialogs: Vec::new(),

      add_or_edit_account_dialog: None,

      account_detail_dialog: None,

      account_list_dialog: None,

      new_main_password_dialog: None,

      main_password_dialog: None,

      help_dialog: None,

      header: widget::Header::new(initial_tree_mode, available_languages),

      working_area: widget::WorkingArea::new(database.clone(), initial_tree_mode),

      status_bar: widget::StatusBar::new(database.clone()),

      temp_password: String::new(),

      exit_code,
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::HeaderMessage(msg) => {
        match msg {
          widget::HeaderMessage::OnTreeModeToggled(toggled) => {
            self.config.set_tree_mode(toggled);
            self.header.update(msg);
            self.working_area.update(widget::WorkingAreaMessage::TreeModeUpdated(toggled));
            Task::none()
          }
          widget::HeaderMessage::OnNewButtonPress => {
            self.update(Message::NewDatabase)
          }
          widget::HeaderMessage::OnLoadButtonPress => {
            self.update(Message::LoadDatabase)
          }
          widget::HeaderMessage::OnSaveButtonPress => {
            self.update(Message::SaveDatabase)
          }
          widget::HeaderMessage::OnSaveAsButtonPress => {
            self.update(Message::SaveAsDatabase)
          }
          widget::HeaderMessage::OnDebugPrintDatabaseButtonPress => {
            let db = &self.database;
            info!("{db:?}");
            Task::none()
          }
          widget::HeaderMessage::OnHelpButtonPress => {
            self.help_dialog = Some(widget::HelpDialog::new());
            Task::none()
          }
          widget::HeaderMessage::OnLanguageSelected(language) => {
            if language.code() != self.i18n.current_language() {
              self.add_confirm_dialog(
                self.i18n.translate("confirm_dialog.title.change_language"),
                self.i18n.translate("confirm_dialog.content.change_language"),
                Message::ChangeLanguageConfirmed(language),
                Message::Noop
              );
            }
            Task::none()
          }
        }
      }

      // wrapping:   TableViewMessage  -> WorkingAreaMessage -> RootWidgetMessage
      // unwrapping: RootWidgetMessage -> WorkingAreaMessage -> TableViewMessage
      Message::WorkingAreaMessage(msg) => {
        enum LocalAction {
          ChildrenAccountList(usize),
          ReferenceAccountList(usize),
          ReferencedByAccountList(usize),
          Add,
          AddChild(usize),
          Detail(usize),
          Modify(usize),
          Delete(usize),
          Noop,
        }

        let local_action =
        if let widget::WorkingAreaMessage::TableViewMessage(msg) = msg {
          if let widget::WorkingAreaTableViewMessage::OnChildrenAccountPress(id) = msg {
            LocalAction::ChildrenAccountList(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnReferenceAccountPress(id) = msg {
            LocalAction::ReferenceAccountList(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnReferencedByAccountPress(id) = msg {
            LocalAction::ReferencedByAccountList(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnAccountDetailPress(id) = msg {
            LocalAction::Detail(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnAccountModifyPress(id) = msg {
            LocalAction::Modify(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnAccountDeletePress(id) = msg {
            LocalAction::Delete(id)
          }
          else if let widget::WorkingAreaTableViewMessage::OnAddAccountPress = msg {
            LocalAction::Add
          }
          else {
            let repack = widget::WorkingAreaMessage::TableViewMessage(msg);
            self.working_area.update(repack);
            LocalAction::Noop
          }
        }
        else if let widget::WorkingAreaMessage::TreeViewMessage(msg) = msg {
          if let widget::WorkingAreaTreeViewMessage::OnAddAccountPress = msg {
            LocalAction::Add
          }
          else if let widget::WorkingAreaTreeViewMessage::OnReferenceAccountPress(id) = msg {
            LocalAction::ReferenceAccountList(id)
          }
          else if let widget::WorkingAreaTreeViewMessage::OnReferencedByAccountPress(id) = msg {
            LocalAction::ReferencedByAccountList(id)
          }
          else if let widget::WorkingAreaTreeViewMessage::OnAddChildAccountPress(parent_id) = msg {
            LocalAction::AddChild(parent_id)
          }
          else if let widget::WorkingAreaTreeViewMessage::OnAccountDetailPress(id) = msg {
            LocalAction::Detail(id)
          }
          else if let widget::WorkingAreaTreeViewMessage::OnAccountModifyPress(id) = msg {
            LocalAction::Modify(id)
          }
          else if let widget::WorkingAreaTreeViewMessage::OnAccountDeletePress(id) = msg {
            LocalAction::Delete(id)
          }
          else {
            let repack = widget::WorkingAreaMessage::TreeViewMessage(msg);
            self.working_area.update(repack);
            LocalAction::Noop
          }
        }
        else {
          self.working_area.update(msg);
          LocalAction::Noop
        };

        match local_action {
          LocalAction::ChildrenAccountList(id) => {
            if let Some(database) = self.database.borrow().as_ref() {
              let children = Vec::from_iter(
                account_util::guarantee_account_from_database(id, database)
                .children_accounts()
                .iter().map(|id| *id)
              );

              self.account_list_dialog = Some(widget::AccountListDialog::new(
                self.i18n.translate_variable(
                  "account_list_dialog.title",
                  &[
                    ("$type", &self.i18n.translate("account_list_dialog.title_type_children")),
                  ]
                ),
                children,
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::ChildrenAccountList while database is None");
            }
          }
          LocalAction::ReferenceAccountList(id) => {
            if let Some(database) = self.database.borrow().as_ref() {
              let reference = Vec::from_iter(
                account_util::guarantee_account_from_database(id, database)
                .reference_accounts()
                .iter().map(|id| *id)
              );

              self.account_list_dialog = Some(widget::AccountListDialog::new(
                self.i18n.translate_variable(
                  "account_list_dialog.title",
                  &[
                    ("$type", &self.i18n.translate("account_list_dialog.title_type_reference")),
                  ]
                ),
                reference,
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::ReferenceAccountList while database is None");
            }
          }
          LocalAction::ReferencedByAccountList(id) => {
            if let Some(database) = self.database.borrow().as_ref() {
              let referenced_by = Vec::from_iter(
                account_util::guarantee_account_from_database(id, database)
                .referenced_by_accounts()
                .iter().map(|id| *id)
              );

              self.account_list_dialog = Some(widget::AccountListDialog::new(
                self.i18n.translate_variable(
                  "account_list_dialog.title",
                  &[
                    ("$type", &self.i18n.translate("account_list_dialog.title_type_referenced_by")),
                  ]
                ),
                referenced_by,
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::ReferencedByAccountList while database is None");
            }
          }
          LocalAction::Add => {
            if let Some(_) = self.database.borrow().as_ref() {
              self.add_or_edit_account_dialog = Some(widget::AddOrEditAccountDialog::new(
                widget::AddOrEditAccountDialogMode::Add,
                None,
                None,
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::Add while database is None");
            }
          }
          LocalAction::AddChild(parent_id) => {
            if let Some(_) = self.database.borrow().as_ref() {
              self.add_or_edit_account_dialog = Some(widget::AddOrEditAccountDialog::new(
                widget::AddOrEditAccountDialogMode::Add,
                None,
                Some(parent_id),
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::Add while database is None");
            }
          }
          LocalAction::Detail(id) => {
            if let Some(_) = self.database.borrow().as_ref() {
              self.account_detail_dialog = Some(widget::AccountDetailDialog::new(id));
            }
            else {
              panic!("Working area message categorized into LocalAction::Detail while database is None");
            }
          }
          LocalAction::Modify(id) => {
            if let Some(database) = self.database.borrow().as_ref() {
              let old_account = database.accounts().get(id)
                .expect(&format!("Old account id ({id}) out of bounds"))
                .as_ref()
                .expect(&format!("Old account is deleted (id={id})"));
              self.add_or_edit_account_dialog = Some(widget::AddOrEditAccountDialog::new(
                widget::AddOrEditAccountDialogMode::Edit,
                Some(old_account),
                None,
              ));
            }
            else {
              panic!("Working area message categorized into LocalAction::Modify while database is None");
            }
          }
          LocalAction::Delete(id) => {
            // basic validation
            let database = self.database.borrow();
            let account = match database.as_ref() {
              Some(db) => {
                match db.accounts().get(id) {
                  Some(account) => {
                    match account {
                      Some(account) => {
                        if account.id() != id {
                          panic!("Account's id ({}) is not identical to it's index ({})", account.id(), id);
                        }
                        else {
                          account
                        }
                      }
                      None => {
                        panic!("Account (id={id}) has already been deleted");
                      }
                    }
                  }
                  None => {
                    panic!("Account index ({id}) out of range");
                  }
                }
              }
              None => {
                panic!("Working area message categorized into LocalAction::Delete while database is None");
              }
            };

            // no children & not referenced by other accounts
            if !account.children_accounts().is_empty() || !account.referenced_by_accounts().is_empty() {
              drop(database);
              self.add_popup_dialog(
                self.i18n.translate("popup_dialog.title.fail_to_delete_account_be_depended"),
                self.i18n.translate("popup_dialog.content.fail_to_delete_account_be_depended"),
                widget::PopupDialogType::Warning,
              );
            }
            else {
              drop(database);
              self.add_confirm_dialog(
                self.i18n.translate("confirm_dialog.title.delete_account"),
                self.i18n.translate_variable("confirm_dialog.content.delete_account", &[("$id", &id.to_string())]),
                Message::DeleteAccountConfirmed(id),
                Message::Noop,
              );
            }
          }
          LocalAction::Noop => {}
        }

        Task::none()
      }

      Message::StatusBarMessage(msg) => {
        self.status_bar.update(msg);
        Task::none()
      }

      Message::PopupDialogMessage(msg) => {
        match msg {
          widget::PopupDialogMessage::OnOkButtonPress(id) => {
            self.popup_dialogs.remove(id);
            Task::none()
          }
        }
      }

      Message::ConfirmDialogMessage(msg) => {
        match msg {
          widget::ConfirmDialogMessage::OnConfirmButtonPress(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_confirm_message();
            self.update(next_msg)
          }
          widget::ConfirmDialogMessage::OnCancelButtonPress(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_cancel_message();
            self.update(next_msg)
          }
        }
      }

      Message::AddOrEditAccountDialogMessage(msg) => {
        if let Some(add_or_edit_account_dialog) = self.add_or_edit_account_dialog.as_mut() {
          match msg {
            widget::AddOrEditAccountDialogMessage::OnConfirmButtonPress => {
              if add_or_edit_account_dialog.validate() {
                let database_update_type = if let Some(database) = self.database.borrow_mut().as_mut() {
                  match add_or_edit_account_dialog.mode() {
                    widget::AddOrEditAccountDialogMode::Add => {
                      // create new account
                      let parent_account = add_or_edit_account_dialog.parent_account();
                      let mut new_account = Account::new(
                        database.accounts().len(),
                        add_or_edit_account_dialog.name().to_string(),
                        parent_account,
                      );
                      for ref_acc in add_or_edit_account_dialog.reference_accounts() {
                        new_account.add_reference_account(*ref_acc);
                      }
                      new_account.set_service(add_or_edit_account_dialog.service().map(String::from));
                      new_account.set_login_name(add_or_edit_account_dialog.login_name().map(String::from));
                      new_account.set_password(add_or_edit_account_dialog.password().map(String::from));
                      new_account.set_comment(add_or_edit_account_dialog.comment().map(String::from));
                      for custom_field in add_or_edit_account_dialog.custom_fields() {
                        new_account.add_custom_field(custom_field.0.clone(), custom_field.1.clone());
                      }

                      // update children accounts
                      if let Some(parent_account_id) = parent_account {
                        let parent_account = database.accounts_mut().get_mut(parent_account_id);
                        match parent_account {
                          Some(parent_account) => {
                            match parent_account {
                              Some(parent_account) => {
                                parent_account.add_children_account(new_account.id());
                              }
                              None => {
                                panic!("parent account (id={parent_account_id}) has been deleted");
                              }
                            }
                          }
                          None => {
                            panic!("parent account id ({parent_account_id}) out of bounds");
                          }
                        }
                      }

                      // update referenced by accounts
                      for reference_account_id in new_account.reference_accounts() {
                        let reference_account = database.accounts_mut().get_mut(*reference_account_id);
                        match reference_account {
                          Some(reference_account) => {
                            match reference_account {
                              Some(reference_account) => {
                                reference_account.add_referenced_by_account(new_account.id());
                              }
                              None => {
                                panic!("reference account (id={reference_account_id}) has been deleted");
                              }
                            }
                          }
                          None => {
                            panic!("reference account id ({reference_account_id}) out of bounds");
                          }
                        }
                      }

                      // push new account into database
                      database.add_account(new_account);

                      if let Some(parent_id) = parent_account {
                        DatabaseUpdatedType::NewChildAccount(parent_id)
                      }
                      else {
                        DatabaseUpdatedType::NewRootAccount
                      }
                    }
                    widget::AddOrEditAccountDialogMode::Edit => {
                      // retrieve old account, temporarily "delete" the old account
                      // to prevent two mutable borrows of database exist at the same time
                      let old_account_id = add_or_edit_account_dialog.id().expect("id is None when editing account");
                      let mut old_account = database.accounts_mut()
                        .get_mut(old_account_id)
                        .expect(&format!("Old account id ({old_account_id}) out of bounds"))
                        .take()
                        .expect(&format!("Old account (id={old_account_id}) is deleted while editing account"));

                      if old_account_id != old_account.id() {
                        panic!("old_account_id ({}) != old_account.id() ({})", old_account_id, old_account.id());
                      }

                      // update old account
                      // update children-parent relationship
                      // first, detach this account (if necessary)
                      if let Some(parent_account_id) = old_account.parent_account() {
                        database.accounts_mut()
                          .get_mut(parent_account_id)
                          .expect(&format!("Parent account id ({parent_account_id}) out of bounds"))
                          .as_mut()
                          .expect(&format!("Parent account (id={parent_account_id}) is deleted"))
                          .remove_children_account(old_account_id);
                      }
                      // then, rebuild children-parent relationship
                      if let Some(parent_account) = add_or_edit_account_dialog.parent_account() {
                        old_account.set_parent_account(Some(parent_account));
                        database.accounts_mut()
                          .get_mut(parent_account)
                          .expect(&format!("Parent account id ({}) out of bounds", parent_account))
                          .as_mut()
                          .expect(&format!("Parent account (id={}) is deleted", parent_account))
                          .add_children_account(old_account_id);
                      }
                      else {
                        old_account.set_parent_account(None);
                      }

                      // update reference relationships
                      // first, remove this account from reference accounts
                      for reference_account_id in old_account.reference_accounts() {
                        database.accounts_mut()
                          .get_mut(*reference_account_id)
                          .expect(&format!("Reference account id ({reference_account_id}) out of bounds"))
                          .as_mut()
                          .expect(&format!("Reference account (id={reference_account_id}) is deleted"))
                          .remove_referenced_by_account(old_account_id);
                      }
                      // then, clear all references
                      old_account.clear_reference_accounts();
                      // finally, rebuild reference relationships
                      for reference_account in add_or_edit_account_dialog.reference_accounts() {
                        old_account.add_reference_account(*reference_account);
                        database.accounts_mut()
                          .get_mut(*reference_account)
                          .expect(&format!("Reference account id ({}) out of bounds", reference_account))
                          .as_mut()
                          .expect(&format!("Reference account (id={}) is deleted", reference_account))
                          .add_referenced_by_account(old_account_id);
                      }

                      old_account.set_name(add_or_edit_account_dialog.name().to_string());
                      old_account.set_service(add_or_edit_account_dialog.service().map(String::from));
                      old_account.set_login_name(add_or_edit_account_dialog.login_name().map(String::from));
                      old_account.set_password(add_or_edit_account_dialog.password().map(String::from));
                      old_account.set_comment(add_or_edit_account_dialog.comment().map(String::from));

                      old_account.clear_custom_fields();
                      for custom_field in add_or_edit_account_dialog.custom_fields() {
                        old_account.add_custom_field(custom_field.0.to_string(), custom_field.1.to_string());
                      }

                      // put modified account back into database
                      if let Some(_) = database.accounts_mut()
                      .get_mut(old_account_id)
                      .expect(&format!("Old account id ({old_account_id}) out of bounds"))
                      .replace(old_account) {
                        panic!("Option::replace should return None as old account has been temporarily deleted");
                      }

                      DatabaseUpdatedType::ModifyAccount(old_account_id)
                    }
                  }
                }
                else {
                  panic!("received AddOrEditAccountDialogMessage while database is None");
                };
                self.add_or_edit_account_dialog = None;
                self.broadcast_database_update(database_update_type);
              }
              else {
                match add_or_edit_account_dialog.mode() {
                  widget::AddOrEditAccountDialogMode::Add => {
                    self.add_popup_dialog(
                      self.i18n.translate("popup_dialog.title.fail_to_add_account"),
                      self.i18n.translate("popup_dialog.content.fail_to_add_account"),
                      widget::PopupDialogType::Warning,
                    )
                  }
                  widget::AddOrEditAccountDialogMode::Edit => {
                    self.add_popup_dialog(
                      self.i18n.translate("popup_dialog.title.fail_to_edit_account"),
                      self.i18n.translate("popup_dialog.content.fail_to_edit_account"),
                      widget::PopupDialogType::Warning,
                    )
                  }
                }
              }
            }
            widget::AddOrEditAccountDialogMessage::OnCancelButtonPress => {
              self.add_or_edit_account_dialog = None;
            }
            other => {
              add_or_edit_account_dialog.update(other);
            }
          }
        }
        else {
          panic!("received AddOrEditAccountDialogMessage while add_or_edit_account_dialog is None");
        }
        Task::none()
      }

      Message::AccountDetailDialogMessage(msg) => {
        match msg {
          widget::AccountDetailDialogMessage::OnCloseButtonPress => {
            self.account_detail_dialog = None;
          }
        }
        Task::none()
      }

      Message::AccountListDialogMessage(msg) => {
        match msg {
          widget::AccountListDialogMessage::OnCloseButtonPress => {
            self.account_list_dialog = None;
          }
        }
        Task::none()
      }

      Message::NewMainPasswordDialogMessage(msg) => {
        match &mut self.new_main_password_dialog {
          Some(new_main_password_dialog) => {
            match msg {
              widget::NewMainPasswordDialogMessage::OnConfirmButtonPress => {
                if new_main_password_dialog.validate() {
                  let (next_msg, password) = self.new_main_password_dialog.take().unwrap().into_on_confirm_message();
                  self.temp_password = password;
                  self.update(next_msg)
                }
                else {
                  Task::none()
                }
              }
              widget::NewMainPasswordDialogMessage::OnCancelButtonPress => {
                let next_msg = self.new_main_password_dialog.take().unwrap().into_on_cancel_message();
                self.update(next_msg)
              },
              other => {
                new_main_password_dialog.update(other);
                Task::none()
              }
            }
          }
          None => {
            panic!("received NewMainPasswordDialogMessage while new_main_password_dialog is None");
          }
        }
      }

      Message::MainPasswordDialogMessage(msg) => {
        match &mut self.main_password_dialog {
          Some(main_password_dialog) => {
            match msg {
              widget::MainPasswordDialogMessage::OnConfirmButtonPress => {
                if main_password_dialog.validate() {
                  let (next_msg, password) = self.main_password_dialog.take().unwrap().into_on_confirm_message();
                  self.temp_password = password;
                  self.update(next_msg)
                }
                else {
                  Task::none()
                }
              }
              widget::MainPasswordDialogMessage::OnCancelButtonPress => {
                let next_msg = self.main_password_dialog.take().unwrap().into_on_cancel_message();
                self.update(next_msg)
              },
              other => {
                main_password_dialog.update(other);
                Task::none()
              }
            }
          }
          None => {
            panic!("received MainPasswordDialogMessage while main_password_dialog is None");
          }
        }
      }

      Message::HelpDialogMessage(msg) => {
        match msg {
          widget::HelpDialogMessage::OnCloseButtonPress => {
            self.help_dialog.take();
            Task::none()
          }
        }
      }

      Message::NewDatabase => {
        if self.database.borrow().is_some() {
          self.add_confirm_dialog(
            self.i18n.translate("confirm_dialog.title.new_database_replace_current_database"),
            self.i18n.translate("confirm_dialog.content.new_database_replace_current_database"),
            Message::NewDatabaseConfirmed,
            Message::Noop
          );
          Task::none()
        }
        else {
          self.update(Message::NewDatabaseConfirmed)
        }
      }

      Message::NewDatabaseConfirmed => {
        Task::perform(
          util::select_new_file(),
          |res| match res {
            Ok(path) => Message::NewDatabaseSelected(path),
            Err(err) => Message::NewDatabaseFail(err.to_string()),
          },
        )
      }

      Message::NewDatabaseSelected(path) => {
        match path {
          Some(path) => {
            self.new_main_password_dialog = Some(widget::NewMainPasswordDialog::new(
              Message::NewDatabaseMainPasswordInputted(path),
              Message::Noop,
            ));
          }
          None => {
          }
        }
        Task::none()
      }

      Message::NewDatabaseMainPasswordInputted(path) => {
        let mut db = Database::new(path, self.temp_password.clone());

        let account_1 = Account::new(0, "Sample Account 1".to_string(), None);
        let mut account_2 = Account::new(1, "Sample Account 2".to_string(), None);
        account_2.add_children_account(2);
        let account_3 = Account::new(2, "Sample Child Account 1".to_string(), Some(1));

        db.add_account(account_1);
        db.add_account(account_2);
        db.add_account(account_3);

        match db.save(&self.i18n) {
          Ok(_) => {
            self.database.replace(Some(db));
            self.update(Message::NewDatabaseSuccess)
          }
          Err(err) => {
            self.update(Message::NewDatabaseFail(err.to_string()))
          }
        }
      }

      Message::NewDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.new_database_success"),
          String::new(),
          widget::PopupDialogType::Success
        );
        self.broadcast_database_update(DatabaseUpdatedType::NewDatabase);
        Task::none()
      }

      Message::NewDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.new_database_fail"),
          err,
          widget::PopupDialogType::Error
        );
        Task::none()
      }

      Message::LoadDatabase => {
        if self.database.borrow().is_some() {
          self.add_confirm_dialog(
            self.i18n.translate("confirm_dialog.title.loaded_database_replace_current_database"),
            self.i18n.translate("confirm_dialog.content.loaded_database_replace_current_database"),
            Message::LoadDatabaseConfirmed,
            Message::Noop
          );
          Task::none()
        }
        else {
          self.update(Message::LoadDatabaseConfirmed)
        }
      }

      Message::LoadDatabaseConfirmed => {
        Task::perform(
          util::select_existing_file(),
          |res| match res {
            Ok(path) => Message::LoadDatabaseSelected(path),
            Err(err) => Message::LoadDatabaseFail(err.to_string()),
          },
        )
      }

      Message::LoadDatabaseSelected(path) => {
        match path {
          Some(path) => {
            self.main_password_dialog = Some(widget::MainPasswordDialog::new(
              Message::LoadDatabaseMainPasswordInputted(path),
              Message::Noop,
            ));
          }
          None => {
          }
        }
        Task::none()
      }

      Message::LoadDatabaseMainPasswordInputted(path) => {
        match Database::load(path, self.temp_password.clone(), &self.i18n) {
          Ok(database) => {
            self.database.replace(Some(database));
            self.update(Message::LoadDatabaseSuccess)
          },
          Err(err) => self.update(Message::LoadDatabaseFail(err.to_string())),
        }
      }

      Message::LoadDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.load_database_success"),
          String::new(),
          widget::PopupDialogType::Success
        );
        self.broadcast_database_update(DatabaseUpdatedType::NewDatabase);
        Task::none()
      }

      Message::LoadDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.load_database_fail"),
          err,
          widget::PopupDialogType::Error
        );
        Task::none()
      }

      Message::SaveDatabase => {
        let result: Option<Result<(), Box<dyn Error>>> = match self.database.borrow_mut().as_mut() {
          Some(database) => {
            match database.save(&self.i18n) {
              Ok(_) => {
                Some(Ok(()))
              }
              Err(err) => {
                Some(Err(err))
              }
            }
          }
          None => {
            None
          }
        };

        if let Some(result) = result {
          match result {
            Ok(_) => {
              self.add_popup_dialog(
                self.i18n.translate("popup_dialog.title.save_database_success"),
                String::new(),
                widget::PopupDialogType::Success
              );
            }
            Err(err) => {
              self.add_popup_dialog(
                self.i18n.translate("popup_dialog.title.save_database_fail"),
                err.to_string(),
                widget::PopupDialogType::Error
              );
            },
          }
        }
        else {
          self.add_popup_dialog(
            self.i18n.translate("popup_dialog.title.no_opened_database"),
            String::new(),
            widget::PopupDialogType::Warning,
          );
        }
        Task::none()
      }

      Message::SaveAsDatabase => {
        if self.database.borrow().is_some() {
          Task::perform(
            util::select_new_file(),
            |res| match res {
              Ok(path) => Message::SaveAsDatabaseSelected(path),
              Err(err) => Message::SaveAsDatabaseFail(err.to_string()),
            },
          )
        }
        else {
          self.add_popup_dialog(
            self.i18n.translate("popup_dialog.title.no_opened_database"),
            String::new(),
            widget::PopupDialogType::Warning,
          );
          Task::none()
        }
      }

      Message::SaveAsDatabaseSelected(path) => {
        match path {
          Some(path) => {
            let result = self.database.borrow_mut().as_mut()
              .expect("`self.database` should be `Some` in `Message::SaveAsDatabaseSelected`")
              .save_as(path, &self.i18n);
            match result {
              Ok(_) => self.update(Message::SaveAsDatabaseSuccess),
              Err(err) => self.update(Message::SaveAsDatabaseFail(err.to_string())),
            }
          }
          None => {
            Task::none()
          }
        }
      }

      Message::SaveAsDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.save_as_database_success"),
          String::new(),
          widget::PopupDialogType::Success
        );
        Task::none()
      }

      Message::SaveAsDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.save_as_database_fail"),
          err.to_string(),
          widget::PopupDialogType::Error
        );
        Task::none()
      }

      Message::DeleteAccountConfirmed(id) => {
        let deleted_account_id = {
          let mut database = self.database.borrow_mut();
          let database = database.as_mut().expect("received WorkingAreaTableViewMessage::OnAddAccountPress while database is None");
          let account = database.accounts_mut().get_mut(id).expect(&format!("Account index ({id}) out of range"));
          if account.is_some() {
            // take the account of out database
            let account = account.take().unwrap();
            // remove dependencies on other accounts
            if let Some(pid) = account.parent_account() {
              database
                .accounts_mut().get_mut(pid).expect(&format!("Parent account id ({pid}) out of range"))
                .as_mut().expect(&format!("Parent account (id={pid}) has already been deleted"))
                .remove_children_account(account.id());
            }
            account.reference_accounts().iter().for_each(|ref_acc| {
              database
                .accounts_mut().get_mut(*ref_acc).expect(&format!("Reference account id ({ref_acc}) out of range"))
                .as_mut().expect(&format!("Reference account (id={ref_acc}) has already been deleted"))
                .remove_referenced_by_account(account.id());
            });

            account.id()
          }
          else {
            panic!("Account (id={id}) has already been deleted");
          }
        };
        self.broadcast_database_update(DatabaseUpdatedType::DeleteAccount(deleted_account_id));
        Task::none()
      }

      Message::ChangeLanguageConfirmed(language) => {
        self.config.set_language(language.code());
        match self.config.save() {
          Ok(_) => {
            *self.exit_code.borrow_mut() = EXIT_CODE_RESTART;
            iced::exit()
          },
          Err(err) => {
            panic!("Fail to change language: {err:?}");
          },
        }
      }

      Message::Noop => {
        Task::none()
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    let content = column![
      self.header.view(&self.i18n, &self.style_variable).map(Message::HeaderMessage),
      self.working_area.view(&self.i18n, &self.style_variable).map(Message::WorkingAreaMessage),
      self.status_bar.view(&self.i18n, &self.style_variable).map(Message::StatusBarMessage),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    if self.popup_dialogs.len() > 0 {
      let last_popup_dialog = self.popup_dialogs.last().expect("popup_dialogs has len > 0 but can not unwrap last() call");
      modal(
        content,
        last_popup_dialog.view(&self.i18n).map(Message::PopupDialogMessage),
        Message::Noop
      )
    }
    else if self.confirm_dialogs.len() > 0 {
      let last_confirm_dialog = self.confirm_dialogs.last().expect("confirm_dialogs has len > 0 but can not unwrap last() call");
      modal(
        content,
        last_confirm_dialog.view(&self.i18n).map(Message::ConfirmDialogMessage),
        Message::Noop
      )
    }
    else if let Some(account_detail_dialog) = self.account_detail_dialog.as_ref() {
      if let Some(database) = self.database.borrow().as_ref() {
        modal(
          content,
          account_detail_dialog.view(&self.i18n, database, &self.style_variable).map(Message::AccountDetailDialogMessage),
          Message::Noop,
        )
      }
      else {
        panic!("database is None while account_detail_dialog is Some, which is meaningless, and shouldn't happen");
      }
    }
    else if let Some(account_list_dialog) = self.account_list_dialog.as_ref() {
      if let Some(database) = self.database.borrow().as_ref() {
        modal(
          content,
          account_list_dialog.view(&self.i18n, database, &self.style_variable).map(Message::AccountListDialogMessage),
          Message::Noop,
        )
      }
      else {
        panic!("database is None while account_list_dialog is Some, which is meaningless, and shouldn't happen");
      }
    }
    else if let Some(add_or_edit_account_dialog) = self.add_or_edit_account_dialog.as_ref() {
      if let Some(database) = self.database.borrow().as_ref() {
        modal(
          content,
          add_or_edit_account_dialog.view(&self.i18n, database, &self.style_variable).map(Message::AddOrEditAccountDialogMessage),
          Message::Noop,
        )
      }
      else {
        panic!("database is None while add_or_edit_account_dialog is Some, which is meaningless, and shouldn't happen");
      }
    }
    else if let Some(new_main_password_dialog) = &self.new_main_password_dialog {
      modal(
        content,
        new_main_password_dialog.view(&self.i18n).map(Message::NewMainPasswordDialogMessage),
        Message::Noop,
      )
    }
    else if let Some(main_password_dialog) = &self.main_password_dialog {
      modal(
        content,
        main_password_dialog.view(&self.i18n).map(Message::MainPasswordDialogMessage),
        Message::Noop,
      )
    }
    else if let Some(help_dialog) = &self.help_dialog {
      modal(
        content,
        help_dialog.view(&self.i18n).map(Message::HelpDialogMessage),
        Message::Noop,
      )
    }
    else {
      content.into()
    }
  }

  pub fn add_popup_dialog(&mut self, title: impl Into<String>, content: impl Into<String>, r#type: widget::PopupDialogType) {
    let id = self.popup_dialogs.len();
    self.popup_dialogs.push(widget::PopupDialog::new(id, title, content, r#type));
  }

  pub fn add_confirm_dialog(&mut self, title: String, content: String, on_confirm: Message, on_cancel: Message) {
    let id = self.confirm_dialogs.len();
    self.confirm_dialogs.push(widget::ConfirmDialog::new(id, title, content, on_confirm, on_cancel));
  }

  /// there may be more widgets which need to be informed when database is updated,
  /// this associated function should be the only entrance of this logic
  fn broadcast_database_update(&mut self, update_type: DatabaseUpdatedType) {
    self.working_area.update(widget::WorkingAreaMessage::DatabaseUpdated(update_type));
  }

}
