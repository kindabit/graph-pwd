mod util;
mod app_error;
mod logging;
mod config;
mod i18n;
mod header;

use std::{error::Error, fs};

use config::Config;
use header::Header;
use i18n::I18n;
use iced::{widget::{column, Column}, Font, Task};
use log::debug;
use logging::setup_logging;

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
}

struct RootWidget {
  config: Config,

  i18n: I18n,

  header: Header,
}

impl RootWidget {
  pub fn new(config: Config, i18n: I18n) -> Self {
    Self {
      config,

      i18n,

      header: Header::new(),
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::HeaderMessage(msg) => self.header.update(msg),
    }
  }

  pub fn view(&self) -> Column<Message> {
    column![
      self.header.view(&self.i18n).map(Message::HeaderMessage),
    ]
  }
}
