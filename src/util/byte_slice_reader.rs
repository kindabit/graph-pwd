use std::{collections::{HashMap, HashSet}, error::Error, io::Read};

use chrono::{DateTime, Local, Utc};

use crate::app_error::AppError;

const BYTES_USIZE: usize = usize::BITS as usize / 8;
const BYTES_I64: usize = i64::BITS as usize / 8;

pub struct ByteSliceReader<'a> {

  slice: &'a [u8],

}

impl <'a> ByteSliceReader<'a> {

  pub fn new(slice: &'a [u8]) -> Self {
    Self {
      slice,
    }
  }

  pub fn read_usize(&mut self) -> Result<usize, Box<dyn Error>> {
    let mut buf: [u8; BYTES_USIZE] = [0; BYTES_USIZE];
    self.slice.read_exact(&mut buf)?;
    let value = usize::from_le_bytes(buf);
    Ok(value)
  }

  pub fn read_i64(&mut self) -> Result<i64, Box<dyn Error>> {
    let mut buf: [u8; BYTES_I64] = [0; BYTES_I64];
    self.slice.read_exact(&mut buf)?;
    let value = i64::from_le_bytes(buf);
    Ok(value)
  }

  pub fn read_u8(&mut self) -> Result<u8, Box<dyn Error>> {
    let mut buf: [u8; 1] = [0; 1];
    self.slice.read_exact(&mut buf)?;
    Ok(buf[0])
  }

  pub fn read_option_usize(&mut self) -> Result<Option<usize>, Box<dyn Error>> {
    let flag = self.read_u8()?;
    if flag == 1 {
      let value = self.read_usize()?;
      Ok(Some(value))
    }
    else if flag == 0 {
      Ok(None)
    }
    else {
      panic!("unexpected flag value: {flag}")
    }
  }

  pub fn read_hashset_usize(&mut self) -> Result<HashSet<usize>, Box<dyn Error>> {
    let len = self.read_usize()?;
    let mut hashset = HashSet::with_capacity(len);
    for _ in 0..len {
      hashset.insert(self.read_usize()?);
    }
    Ok(hashset)
  }

  pub fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
    let len = self.read_usize()?;
    let mut buf = vec![0_u8; len];
    self.slice.read_exact(&mut buf)?;
    let value = String::from_utf8(buf)?;
    Ok(value)
  }

  pub fn read_option_string(&mut self) -> Result<Option<String>, Box<dyn Error>> {
    let flag = self.read_u8()?;
    if flag == 1 {
      let value = self.read_string()?;
      Ok(Some(value))
    }
    else if flag == 0 {
      Ok(None)
    }
    else {
      panic!("unexpected flag value: {flag}")
    }
  }

  pub fn read_hashmap_string_string(&mut self) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let len = self.read_usize()?;
    let mut hashmap = HashMap::new();
    for _ in 0..len {
      let key = self.read_string()?;
      let value = self.read_string()?;
      hashmap.insert(key, value);
    }
    Ok(hashmap)
  }

  pub fn read_datetime_local(&mut self) -> Result<DateTime<Local>, Box<dyn Error>> {
    let timestamp_ms = self.read_i64()?;
    match DateTime::<Utc>::from_timestamp_millis(timestamp_ms) {
      Some(datetime) => {
        Ok(datetime.into())
      }
      None => {
        Err(AppError::boxed(format!("timestamp millis out of range: {timestamp_ms}"), None))
      }
    }
  }

  pub fn slice_len(&self) -> usize {
    self.slice.len()
  }

}
