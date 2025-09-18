use std::sync::{Arc, Mutex};

use iced::{widget::{button, combo_box, row, text, toggler, Button, ComboBox, Space}, Alignment, Color, Element, Length, Padding};
use log::warn;

use crate::{font_icon, i18n::{I18n, Language}, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

pub struct Header {

  tree_mode: bool,

  available_languages: combo_box::State<Language>,

}

#[derive(Clone, Debug)]
pub enum Message {
  OnTreeModeToggled(bool),
  OnNewButtonPress,
  OnLoadButtonPress,
  OnSaveButtonPress,
  OnSaveAsButtonPress,
  OnDebugPrintDatabaseButtonPress,
  OnHelpButtonPress,
  OnLanguageSelected(Language),
}

impl Header {
  pub fn new(tree_mode: bool, available_languages: Vec<Language>) -> Self {
    Self {
      tree_mode,
      available_languages: combo_box::State::new(available_languages),
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
      Message::OnHelpButtonPress => {
        warn!("Event {MODULE_PATH}::Message::OnHelpButtonPress should be intercepted");
      }
      Message::OnLanguageSelected(_) => {
        warn!("Event {MODULE_PATH}::Message::OnLanguageSelected should be intercepted");
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

    let space = Space::new(Length::Fill, Length::Fixed(4_f32));

    let language_label = font_icon::language_round();

    let language_combobox = ComboBox::new(
      &self.available_languages,
      &i18n.translate("header.language_combobox.placeholder"),
      Some(
        self.available_languages
          .options()
          .iter()
          .find(|al| al.code() == i18n.current_language())
          .expect("Fail to find current language in available languages")
      ),
      Message::OnLanguageSelected
    )
    .width(180);

    // let debug_print_database_button = button(text("DBG PRT DB"))
    //   .on_press(Message::OnDebugPrintDatabaseButtonPress);

    let mut help_button = Button::new(
      font_icon::help_outline_round().size({ StyleVariable::lock(style_variable).header_help_button_font_size })
    )
    .padding({ StyleVariable::lock(style_variable).header_help_button_padding })
    .on_press(Message::OnHelpButtonPress);

    {
      let style_variable = style_variable.clone();
      help_button = help_button.style(move |_theme, status| {
        match status {
          button::Status::Active => {
            iced::widget::button::Style {
              background: Some({ StyleVariable::lock(&style_variable).header_help_button_background }),
              text_color: Color::WHITE,
              ..Default::default()
            }
          }
          button::Status::Hovered => {
            iced::widget::button::Style {
              background: Some({ StyleVariable::lock(&style_variable).header_help_button_hovered_background }),
              text_color: Color::WHITE,
              ..Default::default()
            }
          }
          button::Status::Pressed => {
            iced::widget::button::Style {
              background: Some({ StyleVariable::lock(&style_variable).header_help_button_pressed_background }),
              text_color: Color::WHITE,
              ..Default::default()
            }
          }
          button::Status::Disabled => {
            panic!("Help button is not expected to be disabled");
          }
        }
      });
    }

    let mut header_row = row![
      tree_mode_label,
      tree_mode_toggler,
      new_button,
      load_button,
      save_button,
      save_as_button,
      // debug_print_database_button,
      space,
      help_button,
      language_label,
      language_combobox,
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
