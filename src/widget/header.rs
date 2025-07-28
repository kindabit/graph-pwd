use std::sync::{Arc, Mutex};

use iced::{widget::{button, row, text, toggler}, Alignment, Element, Length};
use log::warn;

use crate::{i18n::I18n, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

pub struct Header {

  tree_mode: bool,

}

#[derive(Clone, Debug)]
pub enum Message {
  OnTreeModeToggled(bool),
  OnNewButtonPress,
  OnLoadButtonPress,
  OnSaveButtonPress,
  OnSaveAsButtonPress,
  OnDebugPrintDatabaseButtonPress,
}

impl Header {
  pub fn new(tree_mode: bool) -> Self {
    Self {
      tree_mode,
    }
  }

  pub fn update(&mut self, msg: Message) {
    match msg {
      Message::OnTreeModeToggled(value) => {
        self.tree_mode = value;
      }
      Message::OnNewButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnNewButtonPress should be intercepted");
      }
      Message::OnLoadButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnLoadButtonPress should be intercepted");
      }
      Message::OnSaveButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnSaveButtonPress should be intercepted");
      }
      Message::OnSaveAsButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnSaveAsButtonPress should be intercepted");
      }
      Message::OnDebugPrintDatabaseButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnDebugPrintDatabaseButtonPress should be intercepted");
      }
    }
  }

  pub fn view(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let tree_mode_label = text(i18n.translate("header.tree_mode_label"));

    let tree_mode_toggler = toggler(self.tree_mode)
      .on_toggle(|toggled| Message::OnTreeModeToggled(toggled));

    let new_button = button(text(i18n.translate("header.new_button")))
      .on_press(Message::OnNewButtonPress);

    let load_button = button(text(i18n.translate("header.load_button")))
      .on_press(Message::OnLoadButtonPress);

    let save_button = button(text(i18n.translate("header.save_button")))
      .on_press(Message::OnSaveButtonPress);

    let save_as_button = button(text(i18n.translate("header.save_as_button")))
      .on_press(Message::OnSaveAsButtonPress);

    // let debug_print_database_button = button(text("DBG PRT DB"))
    //   .on_press(Message::OnDebugPrintDatabaseButtonPress);

    let mut header_row = row![
      tree_mode_label,
      tree_mode_toggler,
      new_button,
      load_button,
      save_button,
      save_as_button,
      // debug_print_database_button,
    ];

    let style_variable = StyleVariable::lock(style_variable);

    header_row = header_row
    .padding(style_variable.header_padding)
    .spacing(style_variable.header_spacing);

    drop(style_variable);

    header_row
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
  }
}
