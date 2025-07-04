use std::fs;

use chrono::Utc;
use log::error;

use crate::app_error::AppError;

pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
  match fs::metadata("./log") {
    Ok(meta) =>
      if !meta.is_dir() {
        return Err(AppError::boxed("'./log' is not a directory", None));
      }
    Err(err) =>
      match fs::exists("./log") {
        Ok(exists) =>
          match exists {
            true => return Err(AppError::boxed("can not read metadata of './log'", Some(Box::new(err)))),
            false => fs::create_dir("./log")?,
          }
        Err(err) => return Err(AppError::boxed("can not read existence of './log'", Some(Box::new(err)))),
      }
  };

  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "[{}][{}] {}: {}",
        Utc::now().format("%F %T%.3f"),
        record.level(),
        record.target(),
        message
      ))
    })
    .level(log::LevelFilter::Debug)
    .chain(fern::DateBased::new(
      "./log/client.log.",
      "%F"
    ))
    .chain(std::io::stdout())
    .apply()?;

  std::panic::set_hook(Box::new(|info| {
    error!("PANIC");
    error!("    Location:");
    match info.location() {
      Some(location) => {
        error!("        File:   {}", location.file());
        error!("        Line:   {}", location.line());
        error!("        Column: {}", location.column());
      },
      None => {
        error!("        No Location Info");
      },
    }
    error!("    Payload:");
    if let Some(p) = info.payload().downcast_ref::<&str>() {
      error!("        {}", p);
    }
    else if let Some(p) = info.payload().downcast_ref::<String>() {
      error!("        {}", p);
    }
    else {
      error!("        No Payload Available");
    }
    error!("    Backtrace:");
    error!("{:#?}", std::backtrace::Backtrace::force_capture());
  }));

  Ok(())
}
