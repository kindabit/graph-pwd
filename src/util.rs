use std::{error::Error, fs};

pub fn write_fatal_error(err: Box<dyn Error>) -> ! {
  fs::write("./fatal_error.txt", format!("{err:?}")).expect("fail to write to './fatal_error.txt'");
  panic!("fatal error occurred, please see './fatal_error.txt' for details");
}
