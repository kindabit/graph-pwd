mod util;
mod app_error;
mod logging;
mod config;
mod i18n;
mod global_state;
mod style_variable;
mod database;
mod widget;

use std::{error::Error, sync::{Arc, Mutex}};

use config::Config;
use i18n::I18n;
use iced::{widget::column, window::Position, Element, Font, Length, Task};
use log::{debug, info};
use logging::setup_logging;

use crate::{database::{account::Account, Database}, global_state::GlobalState, style_variable::StyleVariable, util::modal};

pub fn main() -> Result<(), Box<dyn Error>> {
  setup_logging()?;

  iced::application(
    || {
      let config = Config::new().expect("fail to initialize config");
      debug!("config: {config:?}");

      let i18n = I18n::new(&config).expect("fail to initialize i18n");
      debug!("i18n: {i18n:?}");

      (
        RootWidget::new(config, i18n),
        Task::none(),
      )
    },
    RootWidget::update,
    RootWidget::view
  )
  .font(include_bytes!("./assets/SourceHanSansSC-Regular.otf"))
  .default_font(Font::with_name("Source Han Sans SC"))
  .title("Graph PWD")
  .window(iced::window::Settings {
    position: Position::Specific([0_f32, 0_f32].into()),
    size: [800_f32, 600_f32].into(),
    maximized: true,
    ..Default::default()
  })
  .run()?;

  Ok(())
}

#[derive(Clone, Debug)]
pub enum Message {

  HeaderMessage(widget::HeaderMessage),
  WorkingAreaMessage(widget::WorkingAreaMessage),
  StatusBarMessage(widget::StatusBarMessage),
  PopupDialogMessage(widget::PopupDialogMessage),
  ConfirmDialogMessage(widget::ConfirmDialogMessage),
  AddOrEditAccountDialogMessage(widget::AddOrEditAccountDialogMessage),

  NewDatabase,
  NewDatabaseConfirmed,
  NewDatabaseSelected(Option<String>),
  NewDatabaseSuccess,
  NewDatabaseFail(String),

  LoadDatabase,
  LoadDatabaseConfirmed,
  LoadDatabaseSelected(Option<String>),
  LoadDatabaseSuccess,
  LoadDatabaseFail(String),

  SaveDatabase,

  SaveAsDatabase,
  SaveAsDatabaseSelected(Option<String>),
  SaveAsDatabaseSuccess,
  SaveAsDatabaseFail(String),

  Noop,

}

struct RootWidget {

  config: Config,

  i18n: I18n,

  global_state: GlobalState,

  style_variable: Arc<Mutex<StyleVariable>>,

  database: Option<Database>,

  popup_dialogs: Vec<widget::PopupDialog>,

  confirm_dialogs: Vec<widget::ConfirmDialog>,

  add_or_edit_account_dialog: Option<widget::AddOrEditAccountDialog>,

  header: widget::Header,

  working_area: widget::WorkingArea,

  status_bar: widget::StatusBar,

}

impl RootWidget {

  pub fn new(config: Config, i18n: I18n) -> Self {
    Self {
      config,

      i18n,

      global_state: GlobalState::new(),

      style_variable: Arc::new(Mutex::new(StyleVariable::new())),

      database: None,

      popup_dialogs: Vec::new(),

      confirm_dialogs: Vec::new(),

      add_or_edit_account_dialog: None,

      header: widget::Header::new(),

      working_area: widget::WorkingArea::new(),

      status_bar: widget::StatusBar::new(),
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::HeaderMessage(msg) => {
        match msg {
          widget::HeaderMessage::OnTreeModeToggled(toggled) => {
            self.global_state.set_tree_mode(toggled);
            Task::none()
          }
          widget::HeaderMessage::OnNewButtonClicked => {
            self.update(Message::NewDatabase)
          }
          widget::HeaderMessage::OnLoadButtonClicked => {
            self.update(Message::LoadDatabase)
          }
          widget::HeaderMessage::OnSaveButtonClicked => {
            self.update(Message::SaveDatabase)
          }
          widget::HeaderMessage::OnSaveAsButtonClicked => {
            self.update(Message::SaveAsDatabase)
          }
          widget::HeaderMessage::OnDebugPrintDatabaseButtonClicked => {
            let db = &self.database;
            info!("{db:?}");
            Task::none()
          }
          other => {
            self.header.update(other);
            Task::none()
          }
        }
      }

      Message::WorkingAreaMessage(msg) => {
        if let widget::WorkingAreaMessage::TableViewMessage(msg) = msg {
          if let widget::WorkingAreaTableViewMessage::OnAddAccountPressed = msg {
            self.add_or_edit_account_dialog = Some(widget::AddOrEditAccountDialog::new(
              widget::AddOrEditAccountDialogMode::Add,
              None
            ));
          }
          else {
            let repack = widget::WorkingAreaMessage::TableViewMessage(msg);
            self.working_area.update(repack);
          }
        }
        else {
          self.working_area.update(msg);
        }
        Task::none()
      }

      Message::StatusBarMessage(msg) => {
        self.status_bar.update(msg);
        Task::none()
      }

      Message::PopupDialogMessage(msg) => {
        match msg {
          widget::PopupDialogMessage::OnOkButtonClicked(id) => {
            self.popup_dialogs.remove(id);
            Task::none()
          }
        }
      }

      Message::ConfirmDialogMessage(msg) => {
        match msg {
          widget::ConfirmDialogMessage::OnConfirmButtonClicked(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_confirm_message();
            self.update(next_msg)
          }
          widget::ConfirmDialogMessage::OnCancelButtonClicked(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_cancel_message();
            self.update(next_msg)
          }
        }
      }

      Message::AddOrEditAccountDialogMessage(msg) => {
        if let Some(add_or_edit_account_dialog) = self.add_or_edit_account_dialog.as_mut() {
          if let Some(database) = self.database.as_mut() {
            match msg {
              widget::AddOrEditAccountDialogMessage::OnConfirmButtonClicked => {
                if add_or_edit_account_dialog.validate() {
                  match add_or_edit_account_dialog.mode() {
                    widget::AddOrEditAccountDialogMode::Add => {
                      // create new account
                      let mut new_account = Account::new(
                        database.accounts().len(),
                        add_or_edit_account_dialog.name().to_string(),
                        add_or_edit_account_dialog.parent_account().map(|value| value.0)
                      );
                      for ref_acc in add_or_edit_account_dialog.reference_accounts() {
                        new_account.add_reference_account(*ref_acc.0);
                      }
                      new_account.set_service(add_or_edit_account_dialog.service().map(String::from));
                      new_account.set_login_name(add_or_edit_account_dialog.login_name().map(String::from));
                      new_account.set_password(add_or_edit_account_dialog.password().map(String::from));
                      new_account.set_comment(add_or_edit_account_dialog.comment().map(String::from));
                      for custom_field in add_or_edit_account_dialog.custom_fields() {
                        new_account.add_custom_field(custom_field.0.clone(), custom_field.1.clone());
                      }

                      // update children accounts
                      if let Some(parent_account_id) = new_account.parent_account() {
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
                    },
                    widget::AddOrEditAccountDialogMode::Edit => todo!(),
                  }
                  self.add_or_edit_account_dialog = None;
                  self.broadcast_database_update();
                }
                else {
                  self.add_popup_dialog(
                    "popup_dialog.title.fail_to_add_account",
                    "popup_dialog.content.fail_to_add_account",
                    widget::PopupDialogType::Warning,
                  );
                }
              }
              widget::AddOrEditAccountDialogMessage::OnCancelButtonClicked => {
                self.add_or_edit_account_dialog = None;
              }
              other => {
                add_or_edit_account_dialog.update(other);
              }
            }
          }
          else {
            panic!("received AddOrEditAccountDialogMessage while database is None");
          }
        }
        else {
          panic!("received AddOrEditAccountDialogMessage while add_or_edit_account_dialog is None");
        }
        Task::none()
      }

      Message::NewDatabase => {
        if self.database.is_some() {
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
            self.database = Some(Database::new(path));
            let db = self.database.as_mut().unwrap();
            db.add_account(Account::new(0, "Sample Account 1".to_string(), None));
            db.add_account(Account::new(1, "Sample Account 2".to_string(), None));
            db.add_account(Account::new(2, "Sample Child Account 1".to_string(), Some(0)));
            self.update(Message::NewDatabaseSuccess)
          },
          None => Task::none()
        }
      }

      Message::NewDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.new_database_success"),
          String::new(),
          widget::PopupDialogType::Success
        );
        self.broadcast_database_update();
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
        if self.database.is_some() {
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
          Some(path) =>
            match Database::load(path) {
              Ok(database) => {
                self.database = Some(database);
                self.update(Message::LoadDatabaseSuccess)
              },
              Err(err) => self.update(Message::LoadDatabaseFail(err.to_string())),
            }
          ,
          None => Task::none()
        }
      }

      Message::LoadDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.load_database_success"),
          String::new(),
          widget::PopupDialogType::Success
        );
        self.broadcast_database_update();
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
        if let Some(database) = &self.database {
          match database.save() {
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
        if self.database.is_some() {
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
            match self.database.as_mut().expect("`self.database` should be `Some` in `Message::SaveAsDatabaseSelected`").save_as(path) {
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

      Message::Noop => {
        Task::none()
      }
    }
  }

  pub fn view(&self) -> Element<Message> {
    let content = column![
      self.header.view(&self.i18n, &self.global_state, &self.style_variable).map(Message::HeaderMessage),
      self.working_area.view(&self.i18n, self.database.as_ref(), &self.global_state, &self.style_variable).map(Message::WorkingAreaMessage),
      self.status_bar.view(&self.i18n, self.database.as_ref(), &self.style_variable).map(Message::StatusBarMessage),
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
    else if let Some(add_or_edit_account_dialog) = self.add_or_edit_account_dialog.as_ref() {
      if let Some(database) = self.database.as_ref() {
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
  fn broadcast_database_update(&mut self) {
    let database = self.database.as_ref().expect("database is None when broadcasting database update");
    self.working_area.update(
      widget::WorkingAreaMessage::DatabaseUpdated {
        accounts_len: database.accounts().len()
      }
    );
  }

}
