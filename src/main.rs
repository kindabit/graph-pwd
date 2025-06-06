mod util;
mod app_error;
mod logging;
mod config;
mod i18n;
mod database;
mod popup_dialog;
mod confirm_dialog;
mod header;

use std::error::Error;

use config::Config;
use header::Header;
use i18n::I18n;
use iced::{widget::column, Element, Font, Length, Task};
use log::debug;
use logging::setup_logging;

use crate::{confirm_dialog::ConfirmDialog, database::Database, popup_dialog::{PopupDialog, PopupDialogType}, util::modal};

pub fn main() -> Result<(), Box<dyn Error>> {
  setup_logging()?;

  let config = Config::new()?;

  debug!("config: {config:?}");

  let i18n = I18n::new(&config)?;

  debug!("i18n: {i18n:?}");

  iced::application("Graph PWD", RootWidget::update, RootWidget::view)
    .font(include_bytes!("./assets/SourceHanSansSC-Regular.otf"))
    .default_font(Font::with_name("Source Han Sans SC"))
    .run_with(|| (RootWidget::new(config, i18n), Task::none()))?;

  Ok(())
}

#[derive(Clone, Debug)]
pub enum Message {

  HeaderMessage(header::Message),

  PopupDialogMessage(popup_dialog::Message),

  ConfirmDialogMessage(confirm_dialog::Message),

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

  database: Option<Database>,

  popup_dialogs: Vec<PopupDialog>,

  confirm_dialogs: Vec<ConfirmDialog>,

  header: Header,

}

impl RootWidget {

  pub fn new(config: Config, i18n: I18n) -> Self {
    Self {
      config,

      i18n,

      database: None,

      popup_dialogs: Vec::new(),

      confirm_dialogs: Vec::new(),

      header: Header::new(),
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::HeaderMessage(msg) => {
        match self.header.update(msg) {
          Some(msg) => self.update(msg),
          None => Task::none(),
        }
      }

      Message::PopupDialogMessage(msg) => {
        match msg {
          popup_dialog::Message::OnOkButtonClicked(id) => {
            self.popup_dialogs.remove(id);
            Task::none()
          }
        }
      }

      Message::ConfirmDialogMessage(msg) => {
        match msg {
          confirm_dialog::Message::OnConfirmButtonClicked(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_confirm_message();
            self.update(next_msg)
          }
          confirm_dialog::Message::OnCancelButtonClicked(id) => {
            let the_confirm_dialog = self.confirm_dialogs.remove(id);
            let next_msg = the_confirm_dialog.into_on_cancel_message();
            self.update(next_msg)
          }
        }
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
            self.update(Message::NewDatabaseSuccess)
          },
          None => Task::none()
        }
      }

      Message::NewDatabaseSuccess => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.new_database_success"),
          String::new(),
          PopupDialogType::Success
        );
        Task::none()
      }

      Message::NewDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.new_database_fail"),
          err,
          PopupDialogType::Error
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
          PopupDialogType::Success
        );
        Task::none()
      }

      Message::LoadDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.load_database_fail"),
          err,
          PopupDialogType::Error
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
                PopupDialogType::Success
              );
            }
            Err(err) => {
              self.add_popup_dialog(
                self.i18n.translate("popup_dialog.title.save_database_fail"),
                err.to_string(),
                PopupDialogType::Error
              );
            },
          }
        }
        else {
          self.add_popup_dialog(
            self.i18n.translate("popup_dialog.title.no_opened_database"),
            String::new(),
            PopupDialogType::Warning,
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
            PopupDialogType::Warning,
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
          PopupDialogType::Success
        );
        Task::none()
      }

      Message::SaveAsDatabaseFail(err) => {
        self.add_popup_dialog(
          self.i18n.translate("popup_dialog.title.save_as_database_fail"),
          err.to_string(),
          PopupDialogType::Error
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
      self.header.view(&self.i18n).map(Message::HeaderMessage),
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
    else {
      content.into()
    }
  }

  pub fn add_popup_dialog(&mut self, title: String, content: String, r#type: PopupDialogType) {
    let id = self.popup_dialogs.len();
    self.popup_dialogs.push(PopupDialog::new(id, title, content, r#type));
  }

  pub fn add_confirm_dialog(&mut self, title: String, content: String, on_confirm: Message, on_cancel: Message) {
    let id = self.confirm_dialogs.len();
    self.confirm_dialogs.push(ConfirmDialog::new(id, title, content, on_confirm, on_cancel));
  }

}
