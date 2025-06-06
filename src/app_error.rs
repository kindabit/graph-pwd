use std::error::Error;

#[derive(Debug)]
pub struct AppError {

  msg: String,

  source: Option<Box<dyn Error + Send>>,

}

impl AppError {

  pub fn new<T: Into<String>>(msg: T, source: Option<Box<dyn Error + Send>>) -> AppError {
    AppError {
      msg: msg.into(),
      source,
    }
  }

  pub fn boxed<T: Into<String>>(msg: T, source: Option<Box<dyn Error + Send>>) -> Box<dyn Error + Send> {
    Box::new(AppError {
      msg: msg.into(),
      source,
    })
  }

}

impl std::fmt::Display for AppError {

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.msg)
  }

}

impl Error for AppError {

  fn source(&self) -> Option<&(dyn Error + 'static)> {
    if let Some(src) = &self.source {
      Some(src.as_ref())
    }
    else {
      None
    }
  }

  fn cause(&self) -> Option<&dyn Error> {
    self.source()
  }

}
