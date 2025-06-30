use std::sync::{Arc, Mutex};

use iced::{widget::{button, row, text, toggler}, Alignment, Element, Length};

use crate::{global_state::GlobalState, i18n::I18n, style_variable::StyleVariable};

pub struct Header {
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
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn view(&self, i18n: &I18n, global_state: &GlobalState, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let tree_mode_label = text(i18n.translate("header.tree_mode_label"));

    let tree_mode_toggler = toggler(global_state.tree_mode())
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
