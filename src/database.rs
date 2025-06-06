use std::{error::Error, fs};

pub struct Database {

  path: String,

  dummy: Vec<i32>,

}

impl Database {

  pub fn new(path: String) -> Self {
    Self {
      path,
      dummy: vec![2, 3, 5, 7, 11, 13, 17],
    }
  }

  pub fn load(path: String) -> Result<Self, Box<dyn Error>> {
    let data = fs::read_to_string(&path)?;
    let dummy = data.split(' ').into_iter().map(|s| i32::from_str_radix(s, 10).unwrap()).collect();
    Ok(Self {
      path,
      dummy,
    })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let data = self.dummy.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ");
    fs::write(&self.path, data)?;
    Ok(())
  }

  pub fn save_as(&mut self, path: String) -> Result<(), Box<dyn Error>> {
    self.path = path;
    self.save()
  }

}
