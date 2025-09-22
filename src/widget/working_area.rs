pub mod table_view;
pub mod tree_view;

use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use iced::{widget::{container, text}, Alignment, Element, Length};

use crate::{database::Database, i18n::I18n, style_variable::StyleVariable};

use table_view::TableView;
use tree_view::TreeView;

enum WorkingAreaChild {

  Empty,

  TableView(TableView),

  TreeView(TreeView),

}

pub struct WorkingArea {

  child: WorkingAreaChild,

  database: Rc<RefCell<Option<Database>>>,

  tree_mode: bool,

}

#[derive(Clone, Debug)]
pub enum Message {

  TableViewMessage(table_view::Message),

  TreeViewMessage(tree_view::Message),

  DatabaseUpdated(crate::DatabaseUpdatedType),

  TreeModeUpdated(bool),

}

impl WorkingArea {
  pub fn new(database: Rc<RefCell<Option<Database>>>, tree_mode: bool) -> Self {
    let has_database = database.borrow().is_some();

    Self {
      child:
        if has_database {
          if tree_mode {
            WorkingAreaChild::TreeView(TreeView::new(database.clone()))
          }
          else {
            WorkingAreaChild::TableView(TableView::new(database.clone()))
          }
        }
        else {
          WorkingAreaChild::Empty
        },
      database: database.clone(),
      tree_mode,
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::TableViewMessage(msg) => {
        if let WorkingAreaChild::TableView(table_view) = &mut self.child {
          table_view.update(msg)
        }
        else {
          panic!("Received TableViewMessage while child is not TableView");
        }
      }
      Message::TreeViewMessage(msg) => {
        if let WorkingAreaChild::TreeView(tree_view) = &mut self.child {
          tree_view.update(msg)
        }
        else {
          panic!("Received TreeViewMessage while child is not TreeView");
        }
      }
      Message::DatabaseUpdated(update_type) => {
        let database = self.database.borrow();
        let database = database.as_ref();

        if let Some(_database) = database {
          match &mut self.child {
            WorkingAreaChild::Empty => {
              if self.tree_mode {
                self.child = WorkingAreaChild::TreeView(TreeView::new(self.database.clone()));
              }
              else {
                self.child = WorkingAreaChild::TableView(TableView::new(self.database.clone()))
              }
            },
            WorkingAreaChild::TableView(table_view) => {
              table_view.update(table_view::Message::DatabaseUpdated(update_type));
            },
            WorkingAreaChild::TreeView(tree_view) => {
              tree_view.update(tree_view::Message::DatabaseUpdated(update_type));
            },
          }
        }
        else {
          self.child = WorkingAreaChild::Empty
        }
      }
      Message::TreeModeUpdated(tree_mode) => {
        self.tree_mode = tree_mode;
        match &self.child {
          WorkingAreaChild::Empty => {
          }
          WorkingAreaChild::TableView(_) => {
            if tree_mode {
              self.child = WorkingAreaChild::TreeView(TreeView::new(self.database.clone()));
            }
          }
          WorkingAreaChild::TreeView(_) => {
            if !tree_mode {
              self.child = WorkingAreaChild::TableView(TableView::new(self.database.clone()));
            }
          }
        }
      }
    }
  }

  pub fn view(&self, i18n: &I18n, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let container = match &self.child {
      WorkingAreaChild::Empty => {
        container(text(i18n.translate("working_area.no_opened_database")))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
      },
      WorkingAreaChild::TableView(table_view) => {
        container(table_view.view(i18n, style_variable).map(Message::TableViewMessage))
        .width(Length::Fill)
        .height(Length::Fill)
      },
      WorkingAreaChild::TreeView(tree_view) => {
        container(tree_view.view(i18n, style_variable).map(Message::TreeViewMessage))
        .width(Length::Fill)
        .height(Length::Fill)
      },
    };

    container
    .width(Length::Fill)
    .height(Length::Fill)
    .padding({ StyleVariable::lock(style_variable).working_area_padding })
    .into()
  }
}
