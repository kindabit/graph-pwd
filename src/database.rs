pub mod account;

use std::{error::Error, fs};
use account::Account;
use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use log::debug;

use crate::{app_error::AppError, i18n::I18n, util::{number_util, ByteSliceReader, ByteVecWriter}};

#[derive(Debug)]
pub struct Database {

  path: String,

  main_password: String,

  nonce_counter: usize,

  accounts: Vec<Option<Account>>

}

impl Database {

  pub fn new(path: String, main_password: String) -> Self {
    if main_password.is_empty() {
      panic!("Main password is empty");
    }

    Self {
      path,
      main_password,
      nonce_counter: 0,
      accounts: Vec::new(),
    }
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn accounts(&self) -> &Vec<Option<Account>> {
    &self.accounts
  }

  pub fn accounts_mut(&mut self) -> &mut Vec<Option<Account>> {
    &mut self.accounts
  }

  pub fn add_account(&mut self, account: Account) {
    self.accounts.push(Some(account));
  }

  pub fn remove_account(&mut self, account_id: usize) {
    self.accounts[account_id] = None;
  }

  pub fn load(path: String, main_password: String, i18n: &I18n) -> Result<Self, Box<dyn Error>> {
    let data = fs::read(&path)?;

    let nonce_bytes = &data[0..12];
    let nonce_str = match str::from_utf8(nonce_bytes) {
      Ok(nonce_str) => nonce_str,
      Err(err) => panic!("Fail to parse nonce as utf8: {err:?}"),
    };
    if nonce_str.len() != 12 {
      panic!("Nonce string's length is not 12 ({nonce_str})");
    }
    let nonce_num = number_util::string_to_usize_94_12(nonce_str);

    debug!("Nonce number read: {nonce_num}");

    let nonce = Nonce::from_slice(nonce_bytes);

    let key_string = format!("{main_password:0>32}");
    let key_slice = &key_string[0..32];
    let cipher = match Aes256GcmSiv::new_from_slice(key_slice.as_bytes()) {
      Ok(instance) => instance,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_create_cipher_instance"), Some(Box::new(err)))),
    };

    let cipher_data = &data[12..data.len()];

    let plain_data = match cipher.decrypt(nonce, cipher_data) {
      Ok(plain_data) => plain_data,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_decrypt_database"), Some(Box::new(err)))),
    };

    drop(data);

    let mut reader = ByteSliceReader::new(&plain_data);

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
      main_password,
      nonce_counter: nonce_num,
      accounts,
    })
  }

  /// use `&mut self` because need to increase nonce counter
  pub fn save(&mut self, i18n: &I18n) -> Result<(), Box<dyn Error>> {
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

    let nonce_num = self.nonce_counter + 1;
    self.nonce_counter += 1;
    let nonce_string = number_util::usize_to_string_94_12(nonce_num);
    let nonce_bytes = nonce_string.as_bytes();
    let nonce = Nonce::from_slice(nonce_bytes);

    let key_string = format!("{:0>32}", self.main_password);
    let key_slice = &key_string[0..32];
    let cipher = match Aes256GcmSiv::new_from_slice(key_slice.as_bytes()) {
      Ok(instance) => instance,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_create_cipher_instance"), Some(Box::new(err)))),
    };

    let cipher_data = match cipher.encrypt(nonce, data.as_slice()) {
      Ok(cipher_data) => cipher_data,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_encrypt_database"), Some(Box::new(err)))),
    };

    let mut output = Vec::new();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&cipher_data);

    fs::write(&self.path, output)?;

    Ok(())
  }

  pub fn save_as(&mut self, path: String, i18n: &I18n) -> Result<(), Box<dyn Error>> {
    self.path = path;
    self.save(i18n)
  }

}
