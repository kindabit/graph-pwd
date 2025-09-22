use std::sync::{Arc, Mutex};

use iced::{border::Radius, clipboard, widget::{button, combo_box, row, text, toggler, Button, Column, ComboBox, ProgressBar, Space}, Alignment, Border, Color, Element, Length, Task};
use log::warn;

use crate::{font_icon, i18n::{I18n, Language}, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

pub struct Header {

  tree_mode: bool,

  available_languages: combo_box::State<Language>,

  clear_clipboard_scheduled: bool,

  clear_clipboard_countdown: i32,

  clear_clipboard_countdown_config: i32,

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
  pub fn new(
    tree_mode: bool,
    available_languages: Vec<Language>,
    clear_clipboard_countdown_config: i32,
  ) -> Self {
    Self {
      tree_mode,
      available_languages: combo_box::State::new(available_languages),
      clear_clipboard_scheduled: false,
      clear_clipboard_countdown: 0,
      clear_clipboard_countdown_config,
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

    header_row = header_row
    .padding({ StyleVariable::lock(style_variable).header_padding })
    .spacing({ StyleVariable::lock(style_variable).header_spacing })
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Shrink);

    Column::new()
    .push(header_row)
    .push(
      if self.clear_clipboard_scheduled {
        let style_variable = style_variable.clone();
        ProgressBar::new(
          0_f32..=1_f32,
          self.clear_clipboard_countdown as f32 / self.clear_clipboard_countdown_config as f32,
        )
        .style(move |_theme| {
          iced::widget::progress_bar::Style {
            background: { StyleVariable::lock(&style_variable).header_progress_bar_active_background },
            bar: { StyleVariable::lock(&style_variable).header_progress_bar_active_bar },
            border: Border {
              color: Color::TRANSPARENT,
              width: 0_f32,
              radius: Radius::new(0),
            }
          }
        })
      }
      else {
        let style_variable = style_variable.clone();
        ProgressBar::new(
          0_f32..=1_f32,
          0_f32
        )
        .style(move |_theme| {
          iced::widget::progress_bar::Style {
            background: { StyleVariable::lock(&style_variable).header_progress_bar_inactive_background },
            bar: { StyleVariable::lock(&style_variable).header_progress_bar_inactive_bar },
            border: Border {
              color: Color::TRANSPARENT,
              width: 0_f32,
              radius: Radius::new(0),
            }
          }
        })
      }
      .girth({ StyleVariable::lock(style_variable).header_progress_bar_height })
    )
    .into()
  }

  pub fn schedule_clear_clipboard(&mut self) {
    self.clear_clipboard_scheduled = true;
    self.clear_clipboard_countdown = self.clear_clipboard_countdown_config;
  }

  pub fn lapse1s(&mut self) -> Task<Message> {
    if self.clear_clipboard_scheduled {
      self.clear_clipboard_countdown -= 1;
      if self.clear_clipboard_countdown <= 0 {
        self.clear_clipboard_scheduled = false;
        clipboard::write(String::new())
      }
      else {
        Task::none()
      }
    }
    else {
      Task::none()
    }
  }

}
