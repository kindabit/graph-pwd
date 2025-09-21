use std::{cell::RefCell, cmp::min, rc::Rc, sync::{Arc, Mutex}};

use iced::{widget::{container, scrollable, text::Wrapping, Button, Checkbox, Column, Container, MouseArea, PickList, Row, Space, Text, TextInput}, Alignment, Element, Length};
use log::warn;

use crate::{database::{account::Account, Database}, font_icon, i18n::I18n, style_variable::StyleVariable, util::filter_util};

const MODULE_PATH: &str = module_path!();

pub struct TableView {

  database: Rc<RefCell<Option<Database>>>,

  filter: String,

  applied_filter: String,

  hide_deleted_accounts: bool,

  total_page_no: usize,

  /// Starts from 1
  page_no: usize,

  page_size: usize,

  available_page_size: [usize; 5],

  jump_to_page_no: usize,

}

#[derive(Clone, Debug)]
pub enum Message {

  DatabaseUpdated(crate::DatabaseUpdatedType),

  OnFilterInputInput(String),

  OnFilterInputEnter,

  OnHideDeletedAccountsToggle(bool),

  OnChildrenAccountPress(usize),

  OnReferenceAccountPress(usize),

  OnReferencedByAccountPress(usize),

  OnAccountDetailPress(usize),

  OnAccountModifyPress(usize),

  OnAccountDeletePress(usize),

  OnAddAccountPress,

  OnPageSizeSelect(usize),

  OnPrevPress,

  OnPostPress,

  OnJumpToInput(String),

  OnJumpToPress,

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

  pub fn new(database: Rc<RefCell<Option<Database>>>) -> Self {
    let mut instance = Self {
      database,
      filter: String::new(),
      applied_filter: String::new(),
      hide_deleted_accounts: true,
      page_no: 1,
      total_page_no: 1,
      page_size: 10,
      available_page_size: [10, 20, 30, 40, 50],
      jump_to_page_no: 1,
    };
    instance.reset_page();
    instance
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::DatabaseUpdated(update_type) => {
        self.reset_page();
      }
      Message::OnFilterInputInput(filter) => {
        self.filter = filter;
      }
      Message::OnFilterInputEnter => {
        self.applied_filter = self.filter.trim().to_lowercase();
        self.reset_page();
      }
      Message::OnHideDeletedAccountsToggle(value) => {
        self.hide_deleted_accounts = value;
        self.reset_page();
      }
      Message::OnChildrenAccountPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnChildrenAccountPress should be intercepted");
      }
      Message::OnReferenceAccountPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnReferenceAccountPress should be intercepted");
      }
      Message::OnReferencedByAccountPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnReferencedByAccountPress should be intercepted");
      }
      Message::OnAccountDetailPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnAccountDetailPress should be intercepted");
      }
      Message::OnAccountModifyPress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnAccountModifyPress should be intercepted");
      }
      Message::OnAccountDeletePress(_id) => {
        warn!("Event {MODULE_PATH}::Message::OnAccountDeletePress should be intercepted");
      }
      Message::OnAddAccountPress => {
        warn!("Event {MODULE_PATH}::Message::OnAddAccountPress should be intercepted");
      }
      Message::OnPageSizeSelect(new_page_size) => {
        self.page_size = new_page_size;
        self.reset_page();
      }
      Message::OnPrevPress => {
        if self.page_no > 1 {
          self.page_no -= 1;
        }
      }
      Message::OnPostPress => {
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
      Message::OnJumpToPress => {
        self.page_no = self.jump_to_page_no;
      }
    }
  }

  pub fn view(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut table = Column::new()
      .push(self.search_box(i18n, style_variable))
      .push(self.head(i18n, style_variable));

    let mut body = Column::new();
    for row in self.body(i18n, style_variable) {
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

  fn search_box(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Row<Message> {
    Row::new()
    .push(
      Text::new(i18n.translate("working_area.table_view.search_box.filter"))
    )
    .push(
      TextInput::new(&i18n.translate("working_area.table_view.search_box.filter_placeholder"), &self.filter)
      .on_input(Message::OnFilterInputInput)
      .on_paste(Message::OnFilterInputInput)
      .on_submit(Message::OnFilterInputEnter)
      .width(Length::FillPortion(1_u16))
    )
    .push(
      Space::new(
        { StyleVariable::lock(style_variable).working_area_table_view_search_box_middle_space_width },
        Length::Fixed(1_f32)
      )
    )
    .push(
      Text::new(i18n.translate("working_area.table_view.search_box.applied_filter"))
    )
    .push(
      Text::new(&self.applied_filter)
      .width(Length::FillPortion(1_u16))
    )
    .push(
      Checkbox::new(i18n.translate("working_area.table_view.search_box.hide_deleted_accounts"), self.hide_deleted_accounts)
      .on_toggle(Message::OnHideDeletedAccountsToggle)
    )
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .padding({ StyleVariable::lock(style_variable).working_area_table_view_search_box_padding })
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

  fn body(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Vec<Container<Message>> {
    let database = self.database.borrow();
    let database = match database.as_ref() {
      Some(db) => {
        db
      },
      None => {
        panic!("Database is None when rendering body of table view");
      }
    };

    let page_start = (self.page_no - 1) * self.page_size;
    let page_end = min(page_start + self.page_size, self.get_filtered_accounts_iter(database.accounts()).count());
    let real_page_size = page_end - page_start;

    let mut rows = Vec::new();

    for (index, account) in self.get_filtered_accounts_iter(database.accounts()).skip(page_start).take(real_page_size) {
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
            self.body_link_cell_common(
              account.children_accounts().len().to_string(),
              style_variable,
              Self::COLUMN_WIDTH[2],
              Message::OnChildrenAccountPress(account.id())
            )
          )
          .push(
            self.body_link_cell_common(
              account.reference_accounts().len().to_string(),
              style_variable,
              Self::COLUMN_WIDTH[3],
              Message::OnReferenceAccountPress(account.id())
            )
          )
          .push(
            self.body_link_cell_common(
              account.referenced_by_accounts().len().to_string(),
              style_variable,
              Self::COLUMN_WIDTH[4],
              Message::OnReferencedByAccountPress(account.id())
            )
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
                .on_press(Message::OnAccountDetailPress(account.id()))
              )
              .push(
                Button::new(Text::new(i18n.translate("working_area.table_view.body.operation.modify")))
                .on_press(Message::OnAccountModifyPress(account.id()))
              )
              .push(
                Button::new(Text::new(i18n.translate("working_area.table_view.body.operation.delete")))
                .on_press(Message::OnAccountDeletePress(account.id()))
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
        Button::new(font_icon::person_add_round())
        .on_press(Message::OnAddAccountPress)
      )
      .push(Space::new(Length::Fill, Length::Fixed(3_f32)))
      .push(Text::new(i18n.translate("working_area.table_view.footer.page_size")))
      .push(
        PickList::new(
          self.available_page_size,
          Some(self.page_size),
          Message::OnPageSizeSelect,
        )
      )
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.prev")))
        .on_press(Message::OnPrevPress)
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
        .on_press(Message::OnPostPress)
      )
      .push(
        TextInput::new("", &self.jump_to_page_no.to_string())
        .width(style_variable.working_area_table_view_footer_jump_to_input_width)
        .on_input(Message::OnJumpToInput)
        .on_paste(Message::OnJumpToInput)
      )
      .push(
        Button::new(Text::new(i18n.translate("working_area.table_view.footer.jump_to")))
        .on_press(Message::OnJumpToPress)
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

  fn body_link_cell_common(&self, s: impl Into<String>, style_variable: &Arc<Mutex<StyleVariable>>, width: Length, message: Message) -> MouseArea<Message> {
    let style_variable = style_variable.clone();

    MouseArea::new(
      container(
        Text::new(s.into())
        .wrapping(Wrapping::None)
        .style(move |_theme| {
          use iced::widget::text::Style;
          Style {
            color: Some(StyleVariable::lock(&style_variable).working_area_link_color)
          }
        })
      )
      .height(Length::Shrink)
      .width(width)
      .align_y(Alignment::Center)
      .clip(true)
    )
    .on_release(message)
    .interaction(iced::mouse::Interaction::Pointer)
  }

  fn reset_page(&mut self) {
    self.page_no = 1;
    self.jump_to_page_no = 1;
    if let Some(db) = self.database.borrow().as_ref() {
      let len = self.get_filtered_accounts_iter(db.accounts()).count();
      if len == 0 {
        self.total_page_no = 1;
      }
      else {
        self.total_page_no = (len - 1) / self.page_size + 1;
      }
    }
    else {
      self.total_page_no = 1;
    }
  }

  fn get_filtered_accounts_iter<'a, 'b>(&'a self, accounts: &'b Vec<Option<Account>>) -> impl Iterator<Item = (usize, &'b Option<Account>)> {
    accounts.iter()
    .enumerate()
    .filter(|(_index, account)| {
      if let Some(account) = account {
        filter_util::is_match(account, &self.applied_filter)
      }
      else {
        if self.hide_deleted_accounts {
          false
        }
        else {
          if self.applied_filter.len() > 0 {
            false
          }
          else {
            true
          }
        }
      }
    })
  }

}
