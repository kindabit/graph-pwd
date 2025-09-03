use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use iced::{widget::{Button, Column, Container, Row, Rule, Space, Text}, Alignment, Background, Border, Color, Element, Length, Padding};
use log::warn;

use crate::{database::Database, i18n::I18n, style_variable::StyleVariable};

const MODULE_PATH: &str = module_path!();

struct AccountTree {

  pub account_id: usize,

  pub has_children: bool,

  /// ```txt
  /// if
  ///   not has_children
  /// then
  ///   this field does not has any effect
  /// else
  ///   if
  ///     children.len() == 0
  ///   then
  ///     this row is folded
  ///   else
  ///     this row is unfolded
  /// ```
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

  OnAccountDetailPress(usize),

  OnAccountModifyPress(usize),

  OnAccountDeletePress(usize),

}

impl TreeView {

  pub fn new(database: Rc<RefCell<Option<Database>>>) -> Self {
    // initialize forest with root accounts, folded
    let mut forest = Vec::new();
    database
    .borrow().as_ref().expect("database is None in TreeView::new()")
    .accounts().iter().for_each(|account| {
      if let Some(account) = account.as_ref() && account.parent_account().is_none() {
        forest.push(AccountTree {
          account_id: account.id(),
          has_children: account.children_accounts().len() > 0,
          children: Vec::new(),
        });
      }
    });

    Self {
      database,
      forest,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::DatabaseUpdated => {
      }
      Message::OnFoldAccountTreePress(id) => {
        let account_tree = self.find_account_tree_mut(id);
        account_tree.children.clear();
      }
      Message::OnUnfoldAccountTreePress(id) => {
        let mut child_accounts = Vec::new();

        {
          let database = self.database.borrow();
          let database = database.as_ref().expect("database is None in TreeView::update()");
          let accounts = database.accounts();

          let parent_account = match accounts.get(id) {
            Some(account) => {
              match account.as_ref() {
                Some(account) => {
                  account
                }
                None => {
                  panic!("Account (id={id}) has already been deleted");
                }
              }
            }
            None => {
              panic!("Account id ({id}) out of bounds");
            }
          };

          parent_account.children_accounts().iter().for_each(|child_account_id| {
            let child_account = match accounts.get(*child_account_id) {
              Some(account) => {
                match account.as_ref() {
                  Some(account) => {
                    account
                  }
                  None => {
                    panic!("Account (id={child_account_id}) has already been deleted");
                  }
                }
              }
              None => {
                panic!("Account id ({child_account_id}) out of bounds");
              }
            };
            child_accounts.push(AccountTree {
              account_id: *child_account_id,
              has_children: child_account.children_accounts().len() > 0,
              children: Vec::new(),
            });
          });
        }

        let parent_account_tree = self.find_account_tree_mut(id);
        parent_account_tree.children = child_accounts;
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
    let mut rows = Column::new();

    if !self.forest.is_empty() {
      let mut tail_stack: Vec<usize> = Vec::new();
      let mut path_stack: Vec<usize> = Vec::new();

      tail_stack.push(self.forest.last().expect("Forest is empty").account_id);

      for account_tree in &self.forest {
        path_stack.push(account_tree.account_id);
        rows = self.render_account_tree(style_variable, account_tree, rows, &mut tail_stack, &mut path_stack);
        path_stack.pop();
      }
    }

    rows.into()
  }

  fn render_account_tree<'a, 'b>(
    &'a self,
    style_variable: &Arc<Mutex<StyleVariable>>,
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
      { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_margin },
      Length::Fill,
    ));
    match account_tree.has_children {
      true => {
        let style_variable = style_variable.clone();
        content = content.push(
          if account_tree.children.len() > 0 {
            Button::new(
              Container::new(Text::new("-"))
              .width(Length::Fill)
              .height(Length::Fill)
              .align_x(Alignment::Center)
              .align_y(Alignment::Center)
            )
            .on_press(Message::OnFoldAccountTreePress(account_tree.account_id))
          }
          else {
            Button::new(
              Container::new(Text::new("+"))
              .width(Length::Fill)
              .height(Length::Fill)
              .align_x(Alignment::Center)
              .align_y(Alignment::Center)
            )
            .on_press(Message::OnUnfoldAccountTreePress(account_tree.account_id))
          }
          .width({ StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_size })
          .height({ StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_size })
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
                    { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_background }
                  },
                  iced::widget::button::Status::Hovered => {
                    { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_hovered_background }
                  },
                  iced::widget::button::Status::Pressed => {
                    { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_pressed_background }
                  },
                  iced::widget::button::Status::Disabled => {
                    panic!("Fold/Unfold button is disabled, which is not expected");
                  },
                }
              ),
              border: Border {
                color: Color::TRANSPARENT,
                width: 0_f32,
                radius: { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_border_radius },
              },
              text_color: Color::WHITE,
              ..Default::default()
            }
          })
        )
      }
      false => {
        content = content.push(
          Text::new(" ").width({ StyleVariable::lock(style_variable).working_area_tree_view_fold_unfold_button_size })
        )
      }
    };
    content = content.push(Space::new(
      { StyleVariable::lock(&style_variable).working_area_tree_view_fold_unfold_button_margin },
      Length::Fill,
    ));

    // account info
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
          Space::new(
            { StyleVariable::lock(style_variable).working_area_tree_view_service_info_left_padding },
            1_f32,
          )
        )
        .push(
          Text::new(service_info)
        );
      }
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

    if account_tree.children.len() > 0 {
      tail_stack.push(account_tree.children.last().expect("Account tree's children is empty").account_id);
      for child_account_tree in &account_tree.children {
        path_stack.push(child_account_tree.account_id);
        rows = self.render_account_tree(style_variable, child_account_tree, rows, tail_stack, path_stack);
        path_stack.pop();
      }
      tail_stack.pop();
    }

    rows
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

}
