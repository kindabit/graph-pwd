As currently `database` has been change from `Option<Database>` to `Rc<RefCell<Option<Database>>>`,
some rules need to be applied to ensure the correctness of the program.

# RULE No.1

The dereferencing of `database` should not leak into child widgets or other `update()` calls in `update()` function.

```rs
// good
pub fn update(&mut self, msg: Message) -> Task<Message> {
  match msg {
    Message::ChildWidgetMessage(msg) => {
      match msg {
        ChildWidgetMessage::MessageInterceptedByParent => {
          if let Some(database) = self.database.borrow().as_ref() { // <-- dereference database only at where it is needed
            // do something with database
          }
        }
        other_messages => {
          self.child_widget.update(other_messages); // <-- other widgets/update() calls are not influenced
        }
      }
      Task::none()
    }
  }
}

// bad
pub fn update(&mut self, msg: Message) -> Task<Message> {
  match msg {
    Message::ChildWidgetMessage(msg) => {
      if let Some(database) = self.database.borrow().as_ref() { // <-- dereference database at an upper level
        match msg {
          ChildWidgetMessage::MessageInterceptedByParent => {
            // do something with database
          }
          other_messages => {
            self.child_widget.update(other_messages); // <-- database leaked into other widgets/update() calls
            // if this child_widget already holds and uses the pointer
            // which points to the database, this leak may cause panic
          }
        }
        Task::none()
      }
    }
  }
}
```

# RULE No.2

Do not use assignment expression When updating `database`, use `replace()` instead.

```rs
// good
self.database.replace(Some(new_database))

// bad
self.database = Rc::new(RefCell::new(Some(new_database)))
// this will cause multiple database instances exist in multiple widgets
```
