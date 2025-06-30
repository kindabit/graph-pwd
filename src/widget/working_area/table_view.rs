use std::{cmp::min, sync::{Arc, Mutex}};

use iced::{widget::{container, scrollable, text::Wrapping, Button, Column, Container, PickList, Row, Space, Text, TextInput}, Alignment, Element, Length};
use log::warn;

use crate::{database::Database, i18n::I18n, style_variable::{StyleVariable}};

const MODULE_PATH: &str = module_path!();

pub struct TableView {

  total_page_no: usize,

  page_no: usize,

  page_size: usize,

  available_page_size: [usize; 5],

  jump_to_page_no: usize,

  cached_database_accounts_len: usize,

}

#[derive(Clone, Debug)]
pub enum Message {

  DatabaseUpdated {
    accounts_len: usize,
  },

  OnAccountModifyPress(usize),

  OnAddAccountPressed,

  OnPageSizeSelected(usize),

  OnPrevPressed,

  OnPostPressed,

  OnJumpToInput(String),

  OnJumpToPressed,

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
      total_page_no: 1,
      page_size: 10,
      available_page_size: [10, 20, 30, 40, 50],
      jump_to_page_no: 1,
      cached_database_accounts_len: 1,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::DatabaseUpdated { accounts_len } => {
        // validate page number & jump to page number
        if (self.page_no - 1) * self.page_size >= accounts_len {
          self.page_no = 1;
        }
        if (self.jump_to_page_no - 1) * self.page_size >= accounts_len {
          self.jump_to_page_no = 1;
        }
        // update total page number
        if accounts_len == 0 {
          self.total_page_no = 1;
        }
        else {
          self.total_page_no = (accounts_len - 1) / self.page_size + 1;
        }

        self.cached_database_accounts_len = accounts_len;
      }
      Message::OnAccountModifyPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnAccountModifyPress should be intercepted");
      }
      Message::OnAddAccountPressed => {
        warn!("Event {MODULE_PATH}::Message::OnAddAccountPressed should be intercepted");
      }
      Message::OnPageSizeSelected(new_page_size) => {
        self.page_size = new_page_size;

        // validate page number & jump to page number (same logic as Message::DatabaseUpdated)
        if (self.page_no - 1) * self.page_size >= self.cached_database_accounts_len {
          self.page_no = 1;
        }
        if (self.jump_to_page_no - 1) * self.page_size >= self.cached_database_accounts_len {
          self.jump_to_page_no = 1;
        }
        // update total page number (same logic as Message::DatabaseUpdated)
        if self.cached_database_accounts_len == 0 {
          self.total_page_no = 1;
        }
        else {
          self.total_page_no = (self.cached_database_accounts_len - 1) / self.page_size + 1;
        }
      }
      Message::OnPrevPressed => {
        if self.page_no > 1 {
          self.page_no -= 1;
        }
      }
      Message::OnPostPressed => {
        if self.page_no < self.total_page_no {
          self.page_no += 1;
        }
      }
      Message::OnJumpToInput(page_no_string) => {
        match usize::from_str_radix(&page_no_string, 10) {
          Ok(page_no) => {
            if page_no < 1 {
              self.jump_to_page_no = 1;
            }
            else if page_no > self.total_page_no {
              self.jump_to_page_no = self.total_page_no;
            }
            else {
              self.jump_to_page_no = page_no;
            }
          }
          Err(err) => {
            warn!("fail to parse jump to: {err:?}");
          }
        }
      }
      Message::OnJumpToPressed => {
        self.page_no = self.jump_to_page_no;
      }
    }
  }

  pub fn view(&self, i18n: &I18n, database: &Database, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut table = Column::new().push(self.head(i18n, style_variable));

    let mut body = Column::new();
    for row in self.body(i18n, database, style_variable) {
      body = body.push(row);
    }

    let style_variable_temp = StyleVariable::lock(style_variable);
    table = table.push(
      scrollable(
        body
        .width(Length::Fill)
        .height(Length::Shrink)
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .direction(
        scrollable::Direction::Vertical(
          scrollable::Scrollbar::new()
          .width(style_variable_temp.working_area_table_view_scrollbar_width)
          .margin(style_variable_temp.working_area_table_view_scrollbar_margin)
          .scroller_width(style_variable_temp.working_area_table_view_scroller_width)
          .anchor(scrollable::Anchor::Start)
        )
      )
    );
    drop(style_variable_temp);

    table = table.push(self.footer(i18n, style_variable));

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
    .style(move |_theme| {
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
              Some(_password) => {
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
                .on_press(Message::OnAccountModifyPress(account.id()))
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
        .style(move |_theme| {
          iced::widget::container::Style {
            background: Some(StyleVariable::lock(&style_variable_closure).working_area_table_view_body_background(index, deleted)),
            ..Default::default()
          }
        })
      );
    }

    rows
  }

  fn footer(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Container<Message> {
    let style_variable = StyleVariable::lock(style_variable);

    Container::new(
      Row::new()
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.add_account")))
        .on_press(Message::OnAddAccountPressed)
      )
      .push(Space::new(Length::Fill, Length::Fixed(3_f32)))
      .push(Text::new(i18n.translate("working_area.table_view.footer.page_size")))
      .push(
        PickList::new(
          self.available_page_size,
          Some(self.page_size),
          Message::OnPageSizeSelected,
        )
      )
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.prev")))
        .on_press(Message::OnPrevPressed)
      )
      .push(
        Text::new(self.page_no.to_string())
      )
      .push(Text::new("/"))
      .push(
        Text::new(self.total_page_no.to_string())
      )
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.post")))
        .on_press(Message::OnPostPressed)
      )
      .push(
        TextInput::new("", &self.jump_to_page_no.to_string())
        .width(style_variable.working_area_table_view_footer_jump_to_input_width)
        .on_input(Message::OnJumpToInput)
        .on_paste(Message::OnJumpToInput)
      )
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.jump_to")))
        .on_press(Message::OnJumpToPressed)
      )
      .width(Length::Fill)
      .height(Length::Shrink)
      .align_y(Alignment::Center)
      .spacing(style_variable.working_area_table_view_footer_spacing)
      .padding(style_variable.working_area_table_view_footer_padding)
    )
  }

  fn head_cell_common(&self, s: impl Into<String>) -> Container<Message> {
    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
    )
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .clip(true)
  }

  fn body_text_cell_common(&self, s: impl Into<String>) -> Container<Message> {
    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
    )
    .height(Length::Shrink)
    .align_y(Alignment::Center)
    .clip(true)
  }

  fn body_link_cell_common(&self, s: impl Into<String>, style_variable: &Arc<Mutex<StyleVariable>>) -> Container<Message> {
    let style_variable = style_variable.clone();

    container(
      Text::new(s.into())
      .wrapping(Wrapping::None)
      .style(move |_theme| {
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
