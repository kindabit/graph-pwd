pub mod table_view;
pub mod tree_view;

use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use iced::{widget::{container, text}, Alignment, Element, Length};

use crate::{database::Database, global_state::GlobalState, i18n::I18n, style_variable::StyleVariable, widget::working_area::table_view::TableView};

pub struct WorkingArea {

  table_view: TableView,

  database: Rc<RefCell<Option<Database>>>,

}

#[derive(Clone, Debug)]
pub enum Message {

  TableViewMessage(table_view::Message),

  DatabaseUpdated,

}

impl WorkingArea {
  pub fn new(database: Rc<RefCell<Option<Database>>>) -> Self {
    Self {
      table_view: TableView::new(database.clone()),
      database: database.clone(),
    }
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::TableViewMessage(msg) => {
        self.table_view.update(msg)
      }
      Message::DatabaseUpdated => {
        self.table_view.update(table_view::Message::DatabaseUpdated)
      }
    }
  }

  pub fn view(&self, i18n: &I18n, global_state: &GlobalState, style_variable: &Arc<Mutex<StyleVariable>>) -> Element<Message> {
    let tree_mode = global_state.tree_mode();

    let container =  match self.database.borrow().as_ref() {
      Some(database) => {
        container(self.table_view.view(i18n, database, style_variable).map(Message::TableViewMessage))
        .width(Length::Fill)
        .height(Length::Fill)
      }
      None => {
        container(text(i18n.translate("working_area.no_opened_database")))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
      }
    };

    container
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
