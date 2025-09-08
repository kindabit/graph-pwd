use std::{cell::RefCell, collections::HashSet, rc::Rc, sync::{Arc, Mutex}};

use iced::{widget::{scrollable, Button, Column, Container, Row, Rule, Space, Text}, Alignment, Border, Color, Element, Length, Padding};
use log::warn;

use crate::{database::{account::Account, Database}, font_icon, i18n::I18n, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

struct AccountTree {

  pub account_id: usize,

  pub folded: bool,

  pub children: Vec<AccountTree>,

}

pub struct TreeView {

  database: Rc<RefCell<Option<Database>>>,

  forest: Vec<AccountTree>,

}

#[derive(Clone, Debug)]
pub enum Message {

  DatabaseUpdated,

  OnFoldAccountTreePress(usize),

  OnUnfoldAccountTreePress(usize),

  OnAddAccountPress,

  OnAccountDetailPress(usize),

  OnAccountModifyPress(usize),

  OnAccountDeletePress(usize),

}

impl TreeView {

  pub fn new(database: Rc<RefCell<Option<Database>>>) -> Self {
    let mut instance = Self {
      database,
      forest: Vec::new(),
    };
    instance.build_forest(&HashSet::new());
    instance
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::DatabaseUpdated => {
        let mut unfolded_accounts = HashSet::new();
        self.traverse_forest(&mut |account_tree| {
          if !account_tree.folded {
            unfolded_accounts.insert(account_tree.account_id);
          }
        });
        self.build_forest(&unfolded_accounts);
      }
      Message::OnFoldAccountTreePress(id) => {
        self.find_account_tree_mut(id).folded = true;
      }
      Message::OnUnfoldAccountTreePress(id) => {
        self.find_account_tree_mut(id).folded = false;
      }
      Message::OnAddAccountPress => {
        warn!("Event {MODULE_PATH}::Message::OnAddAccountPress should be intercepted");
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
    }
  }

  /// For detailed logic about how the tree is drawn, please refer to `tree_render_logic.png` or `tree_render_logic.xlsx`
  pub fn view(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let mut ele = Column::new();

    // todo: search bar

    // scrollable rows
    let mut rows = Column::new();

    if !self.forest.is_empty() {
      let mut tail_stack: Vec<usize> = Vec::new();
      let mut path_stack: Vec<usize> = Vec::new();

      tail_stack.push(self.forest.last().expect("Forest is empty").account_id);

      for account_tree in &self.forest {
        path_stack.push(account_tree.account_id);
        rows = self.render_account_tree(style_variable, i18n, account_tree, rows, &mut tail_stack, &mut path_stack);
        path_stack.pop();
      }
    }

    ele = ele.push(
      scrollable(
        rows
        .width(Length::Fill)
        .height(Length::Shrink)
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .direction(
        scrollable::Direction::Vertical(
          scrollable::Scrollbar::new()
          .width({ StyleVariable::lock(style_variable).working_area_tree_view_scrollbar_width })
          .margin({ StyleVariable::lock(style_variable).working_area_tree_view_scrollbar_margin })
          .scroller_width({ StyleVariable::lock(style_variable).working_area_tree_view_scroller_width })
          .anchor(scrollable::Anchor::Start)
        )
      )
    );

    // footer
    ele = ele.push(
      Container::new(
        Row::new()
        .push(
          Button::new(font_icon::person_add_round())
          .on_press(Message::OnAddAccountPress)
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_y(Alignment::Center)
        .spacing({ StyleVariable::lock(style_variable).working_area_tree_view_footer_spacing })
        .padding({ StyleVariable::lock(style_variable).working_area_tree_view_footer_padding })
      )
    );

    ele.into()
  }

  fn render_account_tree<'a, 'b>(
    &'a self,
    style_variable: &Arc<Mutex<StyleVariable>>,
    i18n: &I18n,
    account_tree: &AccountTree,
    mut rows: Column<'b, Message>,
    tail_stack: &mut Vec<usize>,
    path_stack: &mut Vec<usize>,
  ) -> Column<'b, Message> {
    let mut row = Row::new();

    // vertical rules and indents
    for i in 0..tail_stack.len() {
      if i < tail_stack.len() - 1 {
        let tail_id = tail_stack.get(i).expect("Tail stack out of bounds");
        let path_id = path_stack.get(i).expect("Path stack out of bounds");
        if path_id == tail_id {
          // draw a space
          row = row.push(Space::new(4_f32, Length::Fill));
        }
        else {
          // draw a vertical rule
          row = row.push(Rule::vertical(4_f32));
        }
        // draw an indent
        row = row
        .push(
          Space::new(
            { StyleVariable::lock(style_variable).working_area_tree_view_indent_size as f32 },
            Length::Fill,
          )
        )
      }
      else {
        // draw a vertical rule
        row = row.push(Rule::vertical(4_f32));
      }
    }

    let mut content = Row::new();

    // fold/unfold button or a placeholder text, and the left/right margins around it
    content = content.push(Space::new(
      { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_margin },
      Length::Fill,
    ));
    match account_tree.children.len() {
      0 => {
        content = content.push(
          Text::new(" ").width({ StyleVariable::lock(style_variable).working_area_tree_view_row_button_size })
        )
      }
      1.. => {
        let style_variable = style_variable.clone();
        content = content.push(
          if account_tree.folded {
            self.create_row_button(
              font_icon::keyboard_arrow_right_round(),
              Message::OnUnfoldAccountTreePress(account_tree.account_id),
              &style_variable,
            )
          }
          else {
            self.create_row_button(
              font_icon::keyboard_arrow_down_round(),
              Message::OnFoldAccountTreePress(account_tree.account_id),
              &style_variable,
            )
          }
        )
      }
    };
    content = content.push(Space::new(
      { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_margin },
      Length::Fill,
    ));

    // account info & splitter
    {
      let database = self.database.borrow();
      let database = database.as_ref().expect("Database is None when rendering tree view");
      let accounts = database.accounts();
      let account = accounts.get(account_tree.account_id)
        .expect(&format!("Account id ({}) out of range", account_tree.account_id))
        .as_ref()
        .expect(&format!("Account (id = {}) has already been deleted", account_tree.account_id));

      content = content.push(
        Text::new(account.name().to_string())
      );

      if account.login_name().is_some() || account.service().is_some() {
        let mut service_info = String::new();
        if let Some(login_name) = account.login_name() {
          service_info.push_str(login_name);
        }
        service_info.push_str("@");
        if let Some(service) = account.service() {
          service_info.push_str(service);
        }
        content = content
        .push(
          Container::new(
            Rule::vertical(4)
          )
          .width({ StyleVariable::lock(style_variable).working_area_tree_view_service_info_left_padding })
          .height({ StyleVariable::lock(style_variable).working_area_tree_view_splitter_height })
          .align_x(Alignment::Center)
        )
        .push(
          Text::new(service_info)
        );
      }
    }

    // actions & splitter
    {
      content = content
      .push(
        Container::new(
          Rule::vertical(4)
        )
        .width({ StyleVariable::lock(style_variable).working_area_tree_view_actions_left_padding })
        .height({ StyleVariable::lock(style_variable).working_area_tree_view_splitter_height })
        .align_x(Alignment::Center)
      )
      // detail
      .push(
        super::super::common::create_tooltip(
          self.create_row_button(
            font_icon::more_round(),
            Message::OnAccountDetailPress(account_tree.account_id),
            &style_variable
          ),
          i18n.translate("working_area.tree_view.tooltip.detail"),
          &style_variable,
        )
      )
      .push(Space::new(
        { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_margin },
        Length::Fill,
      ))
      // edit
      .push(
        super::super::common::create_tooltip(
          self.create_row_button(
            font_icon::edit_round(),
            Message::OnAccountModifyPress(account_tree.account_id),
            &style_variable
          ),
          i18n.translate("working_area.tree_view.tooltip.modify"),
          &style_variable,
        )
      )
      .push(Space::new(
        { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_margin },
        Length::Fill,
      ))
      // delete
      .push(
        super::super::common::create_tooltip(
          self.create_row_button(
            font_icon::delete_round(),
            Message::OnAccountDeletePress(account_tree.account_id),
            &style_variable
          ),
          i18n.translate("working_area.tree_view.tooltip.delete"),
          &style_variable,
        )
      );
    }

    let mut content_wrapper_horizontal_rule = Column::new();
    content_wrapper_horizontal_rule = content_wrapper_horizontal_rule.push(content.align_y(Alignment::Center));
    content_wrapper_horizontal_rule = content_wrapper_horizontal_rule.push(Rule::horizontal(1));

    row = row.push(content_wrapper_horizontal_rule);

    rows = rows.push(
      row
      .align_y(Alignment::Center)
      .height({ StyleVariable::lock(style_variable).working_area_tree_view_row_height })
    );

    if !account_tree.folded && account_tree.children.len() > 0 {
      tail_stack.push(account_tree.children.last().expect("Account tree's children is empty").account_id);
      for child_account_tree in &account_tree.children {
        path_stack.push(child_account_tree.account_id);
        rows = self.render_account_tree(style_variable, i18n, child_account_tree, rows, tail_stack, path_stack);
        path_stack.pop();
      }
      tail_stack.pop();
    }

    rows
  }

  fn create_row_button<'a, 'b>(
    &'a self,
    text: Text<'b>,
    message: Message,
    style_variable: &Arc<Mutex<StyleVariable>>,
  ) -> Button<'b, Message> {
    let style_variable = style_variable.clone();
    Button::new(
      Container::new(text)
      .width(Length::Fill)
      .height(Length::Fill)
      .align_x(Alignment::Center)
      .align_y(Alignment::Center)
    )
    .on_press(message)
    .width({ StyleVariable::lock(&style_variable).working_area_tree_view_row_button_size })
    .height({ StyleVariable::lock(&style_variable).working_area_tree_view_row_button_size })
    .padding(Padding {
      top: 0_f32,
      right: 0_f32,
      bottom: 0_f32,
      left: 0_f32,
    })
    .style(move |_theme, _status| {
      iced::widget::button::Style {
        background: Some(
          match _status {
            iced::widget::button::Status::Active => {
              { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_background }
            },
            iced::widget::button::Status::Hovered => {
              { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_hovered_background }
            },
            iced::widget::button::Status::Pressed => {
              { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_pressed_background }
            },
            iced::widget::button::Status::Disabled => {
              panic!("Fold/Unfold button is disabled, which is not expected");
            },
          }
        ),
        border: Border {
          color: Color::TRANSPARENT,
          width: 0_f32,
          radius: { StyleVariable::lock(&style_variable).working_area_tree_view_row_button_border_radius },
        },
        text_color: Color::WHITE,
        ..Default::default()
      }
    })
  }

  fn find_account_tree_mut(&mut self, id: usize) -> &mut AccountTree {
    for account_tree in &mut self.forest {
      match Self::find_account_tree_internal_mut(id, account_tree) {
        Some(account_tree) => {
          return account_tree;
        }
        None => {
        }
      }
    }
    panic!("Can not find account (id={id}) in forest");
  }

  fn find_account_tree_internal_mut(id: usize, account_tree: &mut AccountTree) -> Option<&mut AccountTree> {
    if account_tree.account_id == id {
      return Some(account_tree);
    }
    else if account_tree.children.len() > 0 {
      for account_tree in &mut account_tree.children {
        match Self::find_account_tree_internal_mut(id, account_tree) {
          Some(account_tree) => {
            return Some(account_tree);
          }
          None => {
          }
        }
      }
      return None;
    }
    else {
      return None;
    }
  }

  fn build_forest(&mut self, unfolded_accounts: &HashSet<usize>) {
    // initialize forest with root accounts
    let mut forest = Vec::new();
    let database = self.database.borrow();
    let database = database.as_ref().expect("database is None in TreeView::new()");
    let accounts = database.accounts();
    for account in accounts {
      if let Some(account) = account.as_ref() && account.parent_account().is_none() {
        forest.push(AccountTree {
          account_id: account.id(),
          folded: true,
          children: Vec::new(),
        });
      }
    }

    fn build_tree(account_tree: &mut AccountTree, accounts: &Vec<Option<Account>>, unfolded_accounts: &HashSet<usize>) {
      let account = accounts
        .get(account_tree.account_id)
        .expect(&format!("Account id ({}) out of bound", account_tree.account_id))
        .as_ref()
        .expect(&format!("Account (id={}) has already been deleted", account_tree.account_id));
      let children = account.children_accounts();
      if children.len() == 0 {
        // these are default values
        // account_tree.folded = true;
        // account_tree.children = Vec::new();
      }
      else {
        account_tree.folded = !unfolded_accounts.contains(&account_tree.account_id);
        account_tree.children = children.iter()
          .map(|id| {
            let mut account_tree = AccountTree {
              account_id: *id,
              folded: true,
              children: Vec::new(),
            };
            build_tree(&mut account_tree, accounts, unfolded_accounts);
            account_tree
          })
          .collect();
      }
    }

    for account_tree in &mut forest {
      build_tree(account_tree, accounts, &unfolded_accounts);
    }

    self.forest = forest;
  }

  fn traverse_forest(&self, op: &mut impl FnMut(&AccountTree)) {
    fn traverse_forest_internal(forest: &[AccountTree], op: &mut impl FnMut(&AccountTree)) {
      for account_tree in forest {
        op(account_tree);
        traverse_forest_internal(&account_tree.children, op);
      }
    }
    traverse_forest_internal(&self.forest, op);
  }

}
