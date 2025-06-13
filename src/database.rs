pub mod account;

use std::{error::Error, fs};
use account::Account;

use crate::util::{ByteSliceReader, ByteVecWriter};

#[derive(Debug)]
pub struct Database {

  path: String,

  accounts: Vec<Option<Account>>

}

impl Database {

  pub fn new(path: String) -> Self {
    Self {
      path,
      accounts: Vec::new(),
    }
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn accounts(&self) -> &Vec<Option<Account>> {
    &self.accounts
  }

  pub fn add_account(&mut self, name: impl Into<String>, parent_account: Option<usize>) {
    let account = Account::new(self.accounts.len(), name.into(), parent_account);
    self.accounts.push(Some(account));
  }

  pub fn remove_account(&mut self, account_id: usize) {
    self.accounts[account_id] = None;
  }

  pub fn load(path: String) -> Result<Self, Box<dyn Error>> {
    let data = fs::read(&path)?;
    let mut reader = ByteSliceReader::new(&data);

    let num_accounts = reader.read_usize()?;
    let mut accounts = Vec::with_capacity(num_accounts);

    for _ in 0..num_accounts {
      let exist = reader.read_u8()?;
      if exist == 1 {
        let account = Account::from_reader(&mut reader)?;
        accounts.push(Some(account));
      }
      else {
        accounts.push(None);
      }
    }

    Ok(Self {
      path,
      accounts,
    })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::new();
    let mut writer = ByteVecWriter::new(&mut data);

    let len = self.accounts.len();
    writer.write_usize(len);

    self.accounts.iter().for_each(|account| {
      match account {
        Some(account) => {
          writer.write_u8(1_u8);
          account.write(&mut writer);
        }
        None => {
          writer.write_u8(0_u8);
        }
      }
    });

    fs::write(&self.path, data)?;

    Ok(())
  }

  pub fn save_as(&mut self, path: String) -> Result<(), Box<dyn Error>> {
    self.path = path;
    self.save()
  }

}
