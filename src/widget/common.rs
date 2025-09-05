use std::sync::{Arc, Mutex};

use iced::{widget::{tooltip, Container, Text, Tooltip}, Element};

use crate::style_variable::StyleVariable;

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
