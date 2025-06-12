mod byte_slice_reader;
mod byte_vec_writer;

use std::error::Error;

use iced::{widget::{center, container, mouse_area, opaque, stack}, Color, Element};
use native_dialog::DialogBuilder;

use crate::app_error::AppError;

pub use byte_slice_reader::ByteSliceReader;
pub use byte_vec_writer::ByteVecWriter;

pub async fn select_new_file() -> Result<Option<String>, Box<dyn Error + Send>> {
  let path = match DialogBuilder::file()
    .save_single_file()
    .show() {
      Ok(path) => path,
      Err(err) => {
        // manually convert native_dialog::errors::Error to Box<dyn std::error::Error + Send>
        let err2 = AppError::boxed(err.to_string(), Some(Box::new(err)));
        return Err(err2);
      }
    };

  match path {
    Some(path) =>
      match path.to_str() {
        Some(path) => Ok(Some(path.to_owned())),
        None => Err(AppError::boxed(format!("none-UTF-8 path: {path:?}"), None)),
      }
    ,
    None => Ok(None),
  }
}

pub async fn select_existing_file() -> Result<Option<String>, Box<dyn Error + Send>> {
  let path = match DialogBuilder::file()
    .open_single_file()
    .show() {
      Ok(path) => path,
      Err(err) => {
        // manually convert native_dialog::errors::Error to Box<dyn std::error::Error + Send>
        let err2 = AppError::boxed(err.to_string(), Some(Box::new(err)));
        return Err(err2);
      }
    };

  match path {
    Some(path) =>
      match path.to_str() {
        Some(path) => Ok(Some(path.to_owned())),
        None => Err(AppError::boxed(format!("none-UTF-8 path: {path:?}"), None)),
      }
    ,
    None => Ok(None),
  }
}

pub fn modal<'a, Message>(
  base: impl Into<Element<'a, Message>>,
  content: impl Into<Element<'a, Message>>,
  on_blur: Message,
) -> Element<'a, Message>
where
  Message: Clone + 'a,
{
  stack![
    base.into(),
    opaque(
      mouse_area(center(opaque(content)).style(|_theme| {
        container::Style {
          background: Some(
            Color {
              a: 0.8,
              ..Color::BLACK
            }
            .into(),
          ),
          ..container::Style::default()
        }
      }))
      .on_press(on_blur)
    )
  ]
  .into()
}
