use iced::{widget::{button, row, text, toggler}, Alignment, Element, Length};
use log::warn;

use crate::{global_state::GlobalState, i18n::I18n};

const MODULE_PATH: &str = module_path!();

pub struct Header {
}

#[derive(Clone, Debug)]
pub enum Message {
  OnTreeModeToggled(bool),
  OnNewButtonClicked,
  OnLoadButtonClicked,
  OnSaveButtonClicked,
  OnSaveAsButtonClicked,
  OnDebugPrintDatabaseButtonClicked,
}

impl Header {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::OnTreeModeToggled(_toggled) => {
        warn!("Event {MODULE_PATH}::Message::OnTreeModeToggled should be intercepted");
      }
      Message::OnNewButtonClicked => {
        warn!("Event {MODULE_PATH}::Message::OnNewButtonClicked should be intercepted");
      }
      Message::OnLoadButtonClicked => {
        warn!("Event {MODULE_PATH}::Message::OnLoadButtonClicked should be intercepted");
      }
      Message::OnSaveButtonClicked => {
        warn!("Event {MODULE_PATH}::Message::OnSaveButtonClicked should be intercepted");
      }
      Message::OnSaveAsButtonClicked => {
        warn!("Event {MODULE_PATH}::Message::OnSaveAsButtonClicked should be intercepted");
      }
      Message::OnDebugPrintDatabaseButtonClicked => {
        warn!("Event {MODULE_PATH}::Message::OnDebugPrintDatabaseButtonClicked should be intercepted");
      }
    }
  }

  pub fn view(&self, i18n: &I18n, global_state: &GlobalState) -> Element<Message> {
    let tree_mode_label = text(i18n.translate("header.tree_mode_label"));

    let tree_mode_toggler = toggler(global_state.tree_mode())
      .on_toggle(|toggled| Message::OnTreeModeToggled(toggled));

    let new_button = button(text(i18n.translate("header.new_button")))
      .on_press(Message::OnNewButtonClicked);

    let load_button = button(text(i18n.translate("header.load_button")))
      .on_press(Message::OnLoadButtonClicked);

    let save_button = button(text(i18n.translate("header.save_button")))
      .on_press(Message::OnSaveButtonClicked);

    let save_as_button = button(text(i18n.translate("header.save_as_button")))
      .on_press(Message::OnSaveAsButtonClicked);

    // let debug_print_database_button = button(text("DBG PRT DB"))
    //   .on_press(Message::OnDebugPrintDatabaseButtonClicked);

    row![
      tree_mode_label,
      tree_mode_toggler,
      new_button,
      load_button,
      save_button,
      save_as_button,
      // debug_print_database_button,
    ]
    .padding([12, 24])
    .spacing(6)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
  }
}
