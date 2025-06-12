use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};

const ZERO: [u8; 1] = (0 as u8).to_le_bytes();
const ONE: [u8; 1] = (1 as u8).to_le_bytes();

pub struct ByteVecWriter<'a> {

  ve: &'a mut Vec<u8>,

}

impl <'a> ByteVecWriter<'a> {

  pub fn new(buffer: &'a mut Vec<u8>) -> Self {
    Self {
      ve: buffer
    }
  }

  pub fn write_usize(&mut self, data: usize) {
    self.ve.extend_from_slice(&data.to_le_bytes());
  }

  pub fn write_i64(&mut self, data: i64) {
    self.ve.extend_from_slice(&data.to_le_bytes());
  }

  pub fn write_u8(&mut self, data: u8) {
    self.ve.extend_from_slice(&data.to_le_bytes());
  }

  pub fn write_option_usize(&mut self, data: Option<usize>) {
    match data {
      Some(data) => {
        self.ve.extend_from_slice(&ONE);
        self.write_usize(data);
      }
      None => {
        self.ve.extend_from_slice(&ZERO);
      }
    }
  }

  pub fn write_hashset_usize(&mut self, data: &HashSet<usize>) {
    self.ve.extend_from_slice(&data.len().to_le_bytes());
    data.iter().for_each(|item| {
      self.write_usize(*item);
    });
  }

  pub fn write_string(&mut self, data: &String) {
    let bytes = data.as_bytes();
    self.ve.extend_from_slice(&bytes.len().to_le_bytes());
    self.ve.extend_from_slice(bytes);
  }

  pub fn write_option_string(&mut self, data: &Option<String>) {
    match &data {
      Some(data) => {
        self.ve.extend_from_slice(&ONE);
        self.write_string(data);
      }
      None => {
        self.ve.extend_from_slice(&ZERO);
      }
    }
  }

  pub fn write_hashmap_string_string(&mut self, data: &HashMap<String, String>) {
    self.ve.extend_from_slice(&data.len().to_le_bytes());
    data.iter().for_each(|(key, value)| {
      self.write_string(key);
      self.write_string(value);
    });
  }

  pub fn write_datetime_local(&mut self, data: &DateTime<Local>) {
    self.write_i64(data.to_utc().timestamp_millis());
  }

}
