use std::sync::{Arc, Mutex};

use iced::{widget::{tooltip, Button, Container, Text, Tooltip}, Border, Color, Element};

use crate::{font_icon, style_variable::StyleVariable};

pub fn create_tooltip<'a, Message: 'a>(
  content: impl Into<Element<'a, Message>>,
  text: String,
  style_variable: &Arc<Mutex<StyleVariable>>,
) -> Tooltip<'a, Message> {
  let style_variable_container = style_variable.clone();
  Tooltip::new(
    content,
    Container::new(Text::new(text))
    .padding({ StyleVariable::lock(&style_variable).common_tooltip_padding })
    .style(move |_theme| {
      iced::widget::container::Style {
        background: Some({ StyleVariable::lock(&style_variable_container).common_tooltip_background }),
        border: { StyleVariable::lock(&style_variable_container).common_tooltip_border },
        ..Default::default()
      }
    }),
    tooltip::Position::FollowCursor
  )
}

pub fn create_icon_button<'a, 'b, Message>(text: Text<'a>, style_variable: &'b Arc<Mutex<StyleVariable>>) -> Button<'a, Message> {
  let style_variable = style_variable.clone();
  Button::new(text)
  .style(move |_theme, status| {
    iced::widget::button::Style {
      background: Some(
        match status {
          iced::widget::button::Status::Active => {
            { StyleVariable::lock(&style_variable).common_icon_button_background }
          },
          iced::widget::button::Status::Hovered => {
            { StyleVariable::lock(&style_variable).common_icon_button_hovered_background }
          },
          iced::widget::button::Status::Pressed => {
            { StyleVariable::lock(&style_variable).common_icon_button_pressed_background }
          },
          iced::widget::button::Status::Disabled => {
            panic!("Fold/Unfold button is disabled, which is not expected");
          },
        }
      ),
      border: Border {
        color: Color::TRANSPARENT,
        radius: { StyleVariable::lock(&style_variable).common_icon_button_border_radius },
        ..Default::default()
      },
      text_color: Color::WHITE,
      ..Default::default()
    }
  })
}

pub fn create_censor_switch_button<'b, Message>(censor: bool, style_variable: &Arc<Mutex<StyleVariable>>) -> Button<'static, Message> {
  create_icon_button(
    {
      font_icon::remove_red_eye_round().color(
        if censor {
          StyleVariable::lock(&style_variable).password_censor_switch_on_color
        }
        else {
          StyleVariable::lock(&style_variable).password_censor_switch_off_color
        }
      )
    },
    style_variable
  )
}
