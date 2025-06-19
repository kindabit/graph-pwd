use std::{cmp::min, sync::{Arc, Mutex}};

use iced::{widget::{container, scrollable, text::Wrapping, Button, Column, Container, Row, Text}, Alignment, Element, Length};

use crate::{database::Database, i18n::I18n, style_variable::{StyleVariable}};

pub struct TableView {

  page_no: usize,

  page_size: usize,

}

#[derive(Clone, Debug)]
pub enum Message {
}

impl TableView {

  const COLUMN_WIDTH: [Length; 11] = [
    Length::Fixed(48_f32),
    Length::Fixed(80_f32),
    Length::Fixed(96_f32),
    Length::Fixed(96_f32),
    Length::Fixed(112_f32),
    Length::Fixed(160_f32),
    Length::Fixed(128_f32),
    Length::Fixed(128_f32),
    Length::Fixed(128_f32),
    Length::Fill,
    Length::Fixed(224_f32),
  ];

  pub fn new() -> Self {
    Self {
      page_no: 1,
      page_size: 30,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
    }
  }

  /// todo: pagination
  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut table = Column::new().push(self.head(i18n, style_variable));

    let mut body = Column::new();
    for row in self.body(i18n, database, style_variable) {
      body = body.push(row);
    }

    let style_variable = StyleVariable::lock(style_variable);
    let body = scrollable(
      body
      .width(Length::Fill)
      .height(Length::Shrink)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .direction(
      scrollable::Direction::Vertical(
        scrollable::Scrollbar::new()
        .width(style_variable.working_area_table_view_scrollbar_width)
        .margin(style_variable.working_area_table_view_scrollbar_margin)
        .scroller_width(style_variable.working_area_table_view_scroller_width)
        .anchor(scrollable::Anchor::Start)
      )
    );
    drop(style_variable);

    table = table.push(body);

    table
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }

  fn head(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Container<Message> {
    let style_variable_closure = style_variable.clone();
    let style_variable = StyleVariable::lock(style_variable);

    Container::new(
      Row::new()
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.id"))
        .width(Self::COLUMN_WIDTH[0])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.parent_account"))
        .width(Self::COLUMN_WIDTH[1])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.children_accounts"))
        .width(Self::COLUMN_WIDTH[2])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.reference_accounts"))
        .width(Self::COLUMN_WIDTH[3])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.referenced_by_accounts"))
        .width(Self::COLUMN_WIDTH[4])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.name"))
        .width(Self::COLUMN_WIDTH[5])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.service"))
        .width(Self::COLUMN_WIDTH[6])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.login_name"))
        .width(Self::COLUMN_WIDTH[7])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.password"))
        .width(Self::COLUMN_WIDTH[8])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.comment"))
        .width(Self::COLUMN_WIDTH[9])
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.operation"))
        .width(Self::COLUMN_WIDTH[10])
      )
      .spacing(style_variable.working_area_table_view_head_spacing)
      .width(Length::Fill)
      .height(Length::Shrink)
    )
    .padding(style_variable.working_area_table_view_head_padding)
    .width(Length::Fill)
    .height(Length::Shrink)
    .style(move |theme| {
      iced::widget::container::Style {
        background: Some(StyleVariable::lock(&style_variable_closure).working_area_table_view_head_background),
        ..Default::default()
      }
    })
  }

  fn body(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Vec<Container<Message>> {
    let page_start = (self.page_no - 1) * self.page_size;
    let page_end = min(page_start + self.page_size, database.accounts().len());

    let mut rows = Vec::new();

    for (index, account) in database.accounts()[page_start..page_end].iter().enumerate() {
      let row = match account.as_ref() {
        Some(account) => {
          let row = Row::new()
          .push(
            self.body_text_cell_common(format!("{}", account.id()))
            .width(Self::COLUMN_WIDTH[0])
          )
          .push(
            match account.parent_account() {
              Some(parent_account) => {
                self.body_text_cell_common(format!("{}", parent_account))
              }
              None => {
                self.body_text_cell_common("-")
              }
            }
            .width(Self::COLUMN_WIDTH[1])
          )
          .push(
            self.body_link_cell_common(format!("{}", account.children_accounts().len()), style_variable)
            .width(Self::COLUMN_WIDTH[2])
          )
          .push(
            self.body_link_cell_common(format!("{}", account.reference_accounts().len()), style_variable)
            .width(Self::COLUMN_WIDTH[3])
          )
          .push(
            self.body_link_cell_common(format!("{}", account.referenced_by_accounts().len()), style_variable)
            .width(Self::COLUMN_WIDTH[4])
          )
          .push(
            self.body_text_cell_common(account.name().to_string())
            .width(Self::COLUMN_WIDTH[5])
          )
          .push(
            match account.service() {
              Some(service) => {
                self.body_text_cell_common(service)
              }
              None => {
                self.body_text_cell_common("-")
              }
            }
            .width(Self::COLUMN_WIDTH[6])
          )
          .push(
            match account.login_name() {
              Some(login_name) => {
                self.body_text_cell_common(login_name)
              }
              None => {
                self.body_text_cell_common("-")
              }
            }
            .width(Self::COLUMN_WIDTH[7])
          )
          .push(
            match account.password() {
              Some(password) => {
                self.body_text_cell_common("******")
              }
              None => {
                self.body_text_cell_common("-")
              }
            }
            .width(Self::COLUMN_WIDTH[8])
          )
          .push(
            match account.comment() {
              Some(comment) => {
                self.body_text_cell_common(comment)
              }
              None => {
                self.body_text_cell_common("-")
              }
            }
            .width(Self::COLUMN_WIDTH[9])
          );

          let style_variable = StyleVariable::lock(style_variable);

          row.push(
            Container::new(
              Row::new()
              .push(
                Button::new(Text::new(i18n.translate("working_area.table_view.body.operation.detail")))
              )
              .push(
                Button::new(Text::new(i18n.translate("working_area.table_view.body.operation.modify")))
              )
              .push(
                Button::new(Text::new(i18n.translate("working_area.table_view.body.operation.delete")))
              )
              .spacing(style_variable.working_area_table_view_body_operation_spacing)
              .width(Self::COLUMN_WIDTH[10])
              .height(Length::Shrink)
            )
          )
          .spacing(style_variable.working_area_table_view_body_spacing)
          .width(Length::Fill)
          .height(Length::Shrink)
          .align_y(Alignment::Center)
        }
        None => {
          Row::new()
          .push(
            self.body_text_cell_common(format!("{}", index))
            .width(Length::Fill)
          )
          .width(Length::Fill)
          .height(Length::Shrink)
        }
      };

      let style_variable_closure = style_variable.clone();
      let style_variable = StyleVariable::lock(style_variable);

      let deleted = account.is_none();

      rows.push(
        Container::new(row)
        .padding(style_variable.working_area_table_view_body_padding)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(move |theme| {
          iced::widget::container::Style {
            background: Some(StyleVariable::lock(&style_variable_closure).working_area_table_view_body_background(index, deleted)),
            ..Default::default()
          }
        })
      );
    }

    rows
  }

  pub fn head_cell_common(&self, s: impl Into<String>) -> Container<Message> {
    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
    )
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .clip(true)
  }

  pub fn body_text_cell_common(&self, s: impl Into<String>) -> Container<Message> {
    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
    )
    .height(Length::Shrink)
    .align_y(Alignment::Center)
    .clip(true)
  }

  pub fn body_link_cell_common(&self, s: impl Into<String>, style_variable: &Arc<Mutex<StyleVariable>>) -> Container<Message> {
    let style_variable = style_variable.clone();

    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
      .style(move |theme| {
        use iced::widget::text::Style;
        Style {
          color: Some(StyleVariable::lock(&style_variable).working_area_table_view_body_link_cell_text_color)
        }
      })
    )
    .height(Length::Shrink)
    .align_y(Alignment::Center)
    .clip(true)
  }
}
