pub mod account;
pub mod database;
pub mod database_interface;
pub mod append_buffer;
pub mod read_slice;

use std::{error::Error, fs};
use account::Account;
use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use log::debug;
use sha2::{Digest, Sha256};

use crate::{app_error::AppError, i18n::I18n, util::{number_util, security, ByteSliceReader, ByteVecWriter}};

fn sha256(s: &str) -> [u8; 32] {
  let hashed_raw = Sha256::digest(s.as_bytes());
  if hashed_raw.len() != 32 {
    panic!("Length of hashed (sha256) data != 32");
  }
  let mut hashed = [0_u8; 32];
  hashed.copy_from_slice(&hashed_raw);
  hashed
}

#[derive(Debug)]
pub struct Database {

  path: String,

  main_password: [u8; 32], // hashed

  secondary_password: [u8; 32], // hashed

  secondary_password_nonce: [u8; 12],

  nonce_counter: usize,

  accounts: Vec<Option<Account>>

}

impl Database {

  pub fn new(path: String, main_password: String) -> Self {
    if main_password.is_empty() {
      panic!("Main password is empty");
    }

    let hashed_main_password = sha256(&main_password);

    let current_time = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("Fail to get system time")
      .as_secs();
    let mut secondary_password = current_time.to_string();
    secondary_password.push_str(&main_password);
    let hashed_secondary_password = sha256(&secondary_password);

    // erase the memory used by plain text main password, asap
    security::erase_string(main_password);

    let mut secondary_password_nonce = [0_u8; 12];
    for i in 0_usize..12_usize {
      secondary_password_nonce[i] = rand::random();
    }

    Self {
      path,
      main_password: hashed_main_password,
      secondary_password: hashed_secondary_password,
      secondary_password_nonce,
      nonce_counter: 0,
      accounts: Vec::new(),
    }
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn secondary_password(&self) -> &[u8; 32] {
    &self.secondary_password
  }

  pub fn secondary_password_nonce(&self) -> &[u8; 12] {
    &self.secondary_password_nonce
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

    let hashed_main_password = sha256(&main_password);
    let cipher = match Aes256GcmSiv::new_from_slice(&hashed_main_password) {
      Ok(instance) => instance,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_create_cipher_instance"), Some(Box::new(err)))),
    };

    security::erase_string(main_password);

    let cipher_data = &data[12..data.len()];

    let plain_data = match cipher.decrypt(nonce, cipher_data) {
      Ok(plain_data) => plain_data,
      Err(err) => return Err(AppError::boxed(i18n.translate("database.fail_to_decrypt_database"), Some(Box::new(err)))),
    };

    drop(data);

    let mut reader = ByteSliceReader::new(&plain_data);

    let hashed_secondary_password = reader.read_u8_slice::<32>()?;
    let secondary_password_nonce = reader.read_u8_slice::<12>()?;

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
      main_password: hashed_main_password,
      secondary_password: hashed_secondary_password,
      secondary_password_nonce,
      nonce_counter: nonce_num,
      accounts,
    })
  }

  /// use `&mut self` because need to increase nonce counter
  pub fn save(&mut self, i18n: &I18n) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::new();
    let mut writer = ByteVecWriter::new(&mut data);

    writer.write_u8_slice(&self.secondary_password);
    writer.write_u8_slice(&self.secondary_password_nonce);

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

    self.nonce_counter += 1;
    let nonce_string = number_util::usize_to_string_94_12(self.nonce_counter);
    let nonce_bytes = nonce_string.as_bytes();
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = match Aes256GcmSiv::new_from_slice(&self.main_password) {
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
