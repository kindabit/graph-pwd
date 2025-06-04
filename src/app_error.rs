#[derive(Debug)]
pub struct AppError {
  msg: String,
  source: Option<Box<dyn std::error::Error>>,
}

impl AppError {
  pub fn new<T: Into<String>>(msg: T, source: Option<Box<dyn std::error::Error>>) -> AppError {
    AppError {
      msg: msg.into(),
      source,
    }
  }

  pub fn boxed<T: Into<String>>(msg: T, source: Option<Box<dyn std::error::Error>>) -> Box<dyn std::error::Error> {
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

impl std::error::Error for AppError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    if let Some(src) = &self.source {
      Some(src.as_ref())
    }
    else {
      None
    }
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    self.source()
  }
}
