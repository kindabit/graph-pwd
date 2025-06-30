use std::{collections::HashSet, rc::Rc, sync::{Arc, Mutex}, usize};

use iced::{widget::{scrollable, text::Wrapping, Column, Container, MouseArea, Row, Scrollable, Text}, Element, Length};

use crate::{database::Database, style_variable::StyleVariable};

pub struct MiniAccountSelector {
}

#[derive(Clone, Debug)]
pub enum Message {

  /// (id)
  OnRowClick(usize),

}

impl MiniAccountSelector {

  pub fn new() -> Self {
    Self {
    }
  }

  pub fn view(&self, database: &Database, filter: &str, selected_account_ids: &[usize], style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut selected_account_id_set: HashSet<usize> = HashSet::with_capacity(selected_account_ids.len());
    selected_account_id_set.extend(selected_account_ids);
    let selected_account_id_set = Rc::new(selected_account_id_set);

    let mut filtered_accounts = Column::new();

    if filter.len() > 0 {
      let filter = filter.to_lowercase();

      for account in database.accounts() {
        if account.is_none() {
          continue;
        }
        let account = account.as_ref().unwrap();
        let matched = if account.name().to_lowercase().contains(&filter) {
          true
        }
        else {
          match account.service() {
            Some(service) => {
              if service.to_lowercase().contains(&filter) {
                true
              }
              else {
                false
              }
            }
            None => false,
          }
        };
        if !matched {
          continue
        }

        let style_variable_closure = style_variable.clone();
        let selected_account_id_set_closure = selected_account_id_set.clone();
        let id_closure = account.id();

        let style_variable = StyleVariable::lock(style_variable);

        filtered_accounts = filtered_accounts.push(
          MouseArea::new(
            Container::new(
              Row::new()
              .push(
                Container::new(
                  Text::new(account.id().to_string())
                  .wrapping(Wrapping::None)
                )
                .width(style_variable.mini_account_selector_table_account_id_width)
                .height(Length::Shrink)
                .clip(true)
              )
              .push(
                Container::new(
                  Text::new(account.name().to_string())
                  .wrapping(Wrapping::None)
                )
                .width(Length::FillPortion(1))
                .height(Length::Shrink)
                .clip(true)
              )
              .push(
                Container::new(
                  Text::new(
                    match account.service() {
                      Some(service) => {
                        service.to_string()
                      }
                      None => {
                        String::from("-")
                      }
                    }
                  )
                  .wrapping(Wrapping::None)
                )
                .width(Length::FillPortion(1))
                .height(Length::Shrink)
                .clip(true)
              )
              .width(Length::Fill)
              .height(Length::Shrink)
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(move |_theme| {
              let style_variable_closure = StyleVariable::lock(&style_variable_closure);
              if selected_account_id_set_closure.contains(&id_closure) {
                iced::widget::container::Style {
                  background: Some(style_variable_closure.mini_account_selector_selected_account_background),
                  ..Default::default()
                }
              }
              else {
                iced::widget::container::Style::default()
              }
            })
          )
          .on_press(Message::OnRowClick(account.id()))
        )
      }
    }

    let style_variable = StyleVariable::lock(style_variable);

    Scrollable::new(
      filtered_accounts
      .width(Length::Fill)
      .height(Length::Shrink)
    )
    .width(Length::Fill)
    .height(style_variable.mini_account_selector_height)
    .direction(
      scrollable::Direction::Vertical(
        scrollable::Scrollbar::new()
        .width(style_variable.mini_account_selector_scrollbar_width)
        .margin(style_variable.mini_account_selector_scrollbar_margin)
        .scroller_width(style_variable.mini_account_selector_scroller_width)
        .anchor(scrollable::Anchor::Start)
      )
    )
    .into()
  }

}
