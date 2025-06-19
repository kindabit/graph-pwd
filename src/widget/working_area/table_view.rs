use std::cmp::min;

use iced::{widget::{container, text::Wrapping, Button, Column, Container, Row, Text}, Alignment, Color, Element, Length, Padding};

use crate::{database::Database, i18n::I18n, style_variable::StyleVariable};

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
      page_size: 10,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
    }
  }

  /// todo: pagination
  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &StyleVariable) -> Element<Message> {
    let mut outer_column = Column::new().push(self.head(i18n, style_variable));

    let mut inner_column = Column::new();
    for row in self.body(i18n, database, style_variable) {
      inner_column = inner_column.push(row);
    }
    outer_column = outer_column.push(
      inner_column
      .width(Length::Fill)
      .height(Length::Fill)
    );

    outer_column
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }

  fn head(&self, i18n: &I18n, style_variable: &StyleVariable) -> Container<Message> {
    let working_area_table_view_head_background = style_variable.working_area_table_view_head_background;

    Container::new(
      Row::new()
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.id"))
        .width(Self::COLUMN_WIDTH[0])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.parent_account"))
        .width(Self::COLUMN_WIDTH[1])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.children_accounts"))
        .width(Self::COLUMN_WIDTH[2])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.reference_accounts"))
        .width(Self::COLUMN_WIDTH[3])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.referenced_by_accounts"))
        .width(Self::COLUMN_WIDTH[4])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.name"))
        .width(Self::COLUMN_WIDTH[5])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.service"))
        .width(Self::COLUMN_WIDTH[6])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.login_name"))
        .width(Self::COLUMN_WIDTH[7])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.password"))
        .width(Self::COLUMN_WIDTH[8])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.comment"))
        .width(Self::COLUMN_WIDTH[9])
        .height(Length::Shrink)
      )
      .push(
        self.head_cell_common(i18n.translate("working_area.table_view.title.operation"))
        .width(Self::COLUMN_WIDTH[10])
        .height(Length::Shrink)
      )
      .spacing(style_variable.working_area_table_view_head_spacing)
      .width(Length::Fill)
      .height(Length::Shrink)
    )
    .padding(style_variable.working_area_table_view_head_padding)
    .width(Length::Fill)
    .height(Length::Shrink)
    .style(move |theme| {
      use iced::widget::container::Style;
      Style {
        background: Some(working_area_table_view_head_background),
        ..Default::default()
      }
    })
  }

  fn body(&self, i18n: &I18n, database: &Database, style_variable: &StyleVariable) -> Vec<Container<Message>> {
    let working_area_table_view_body_background_0 = style_variable.working_area_table_view_body_background_0;
    let working_area_table_view_body_background_1 = style_variable.working_area_table_view_body_background_1;

    let page_start = (self.page_no - 1) * self.page_size;
    let page_end = min(page_start + self.page_size, database.accounts().len());

    let mut rows = Vec::new();

    for (index, account) in database.accounts()[page_start..page_end].iter().enumerate() {
      rows.push(
        Container::new(
          match account.as_ref() {
            Some(account) => {
              Row::new()
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
              )
              .push(
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
              .spacing(style_variable.working_area_table_view_body_spacing)
              .width(Length::Fill)
              .height(Length::Shrink)
            }
            None => {
              Row::new()
              .push(
                self.body_text_cell_common(format!("{}", index))
                .width(Self::COLUMN_WIDTH[0])
                .height(Length::Shrink)
              )
              .width(Length::Fill)
              .height(Length::Shrink)
            }
          }
        )
        .padding(style_variable.working_area_table_view_body_padding)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(move |theme| {
          use iced::widget::container::Style;
          Style {
            background: Some(
              match index % 2 {
                0 => {
                  working_area_table_view_body_background_0
                }
                1.. => {
                  working_area_table_view_body_background_1
                }
              }
            ),
            ..Default::default()
          }
        })
      )
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

  pub fn body_link_cell_common(&self, s: impl Into<String>, style_variable: &StyleVariable) -> Container<Message> {
    let working_area_table_view_body_link_cell_text_color = style_variable.working_area_table_view_body_link_cell_text_color;

    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
      .style(move |theme| {
        use iced::widget::text::Style;
        Style {
          color: Some(working_area_table_view_body_link_cell_text_color)
        }
      })
    )
    .height(Length::Shrink)
    .align_y(Alignment::Center)
    .clip(true)
  }
}
