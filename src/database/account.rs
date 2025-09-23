use std::{collections::{BTreeMap, BTreeSet}, error::Error};

use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};
use chrono::{DateTime, Local};
use log::info;

use crate::util::{security, ByteSliceReader, ByteVecWriter};

#[derive(Debug)]
pub struct Password {

  ciphered: Vec<u8>,

  plain: Option<String>,

}

impl Password {

  pub fn plain(&self) -> Option<&String> {
    self.plain.as_ref()
  }

  pub fn decipher(&mut self, secondary_password: &[u8], secondary_password_nonce: &[u8]) {
    info!("Deciphering account password");

    let cipher = match Aes256GcmSiv::new_from_slice(secondary_password) {
      Ok(instance) => {
        instance
      }
      Err(err) => {
        panic!("Fail to create cipher when deciphering account password: {err:?}");
      }
    };

    // a random generated fixed nonce
    let nonce = Nonce::from_slice(secondary_password_nonce);

    let plain_data = match cipher.decrypt(nonce, self.ciphered.as_slice()) {
      Ok(plain_data) => {
        plain_data
      }
      Err(err) => {
        panic!("Fail to decipher account password: {err:?}")
      }
    };

    self.plain = Some(
      match String::from_utf8(plain_data) {
        Ok(plain) => {
          plain
        }
        Err(err) => {
          panic!("Fail to parse deciphered account password as utf8 string: {err:?}")
        }
      }
    )
  }

  pub fn cipher(plain: String, secondary_password: &[u8], secondary_password_nonce: &[u8]) -> Vec<u8> {
    let cipher = match Aes256GcmSiv::new_from_slice(secondary_password) {
      Ok(instance) => {
        instance
      }
      Err(err) => {
        panic!("Fail to create cipher when ciphering account password: {err:?}");
      }
    };

    // a random generated fixed nonce
    let nonce = Nonce::from_slice(secondary_password_nonce);

    let ciphered_data = match cipher.encrypt(nonce, plain.as_bytes()) {
      Ok(ciphered_data) => {
        ciphered_data
      }
      Err(err) => {
        panic!("Fail to cipher account password: {err:?}")
      }
    };

    security::erase_string(plain);

    ciphered_data
  }

}

#[derive(Debug)]
pub struct Account {

  id: usize,

  parent_account: Option<usize>,

  children_accounts: BTreeSet<usize>,

  reference_accounts: BTreeSet<usize>,

  referenced_by_accounts: BTreeSet<usize>,

  name: String,

  service: Option<String>,

  login_name: Option<String>,

  password: Option<Password>,

  comment: Option<String>,

  custom_fields: BTreeMap<String, String>,

  create_time: DateTime<Local>,

  modify_time: DateTime<Local>,

}

impl Account {

  pub fn new(id: usize, name: String, parent_account: Option<usize>) -> Self {
    let now = Local::now();
    Self {
      id,
      parent_account,
      children_accounts: BTreeSet::new(),
      reference_accounts: BTreeSet::new(),
      referenced_by_accounts: BTreeSet::new(),
      name,
      service: None,
      login_name: None,
      password: None,
      comment: None,
      custom_fields: BTreeMap::new(),
      create_time: now.clone(),
      modify_time: now.clone(),
    }
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn parent_account(&self) -> Option<usize> {
    self.parent_account
  }

  pub fn set_parent_account(&mut self, parent_account: Option<usize>) {
    self.parent_account = parent_account;
    self.update_modify_time();
  }

  pub fn children_accounts(&self) -> &BTreeSet<usize> {
    &self.children_accounts
  }

  pub fn exist_children_account(&self, children_account: usize) -> bool {
    self.children_accounts.contains(&children_account)
  }

  pub fn add_children_account(&mut self, children_account: usize) {
    self.children_accounts.insert(children_account);
    self.update_modify_time();
  }

  pub fn remove_children_account(&mut self, children_account: usize) {
    if self.children_accounts.remove(&children_account) == false {
      panic!("Child account (id={children_account}) does not exist");
    }
    self.update_modify_time();
  }

  pub fn reference_accounts(&self) -> &BTreeSet<usize> {
    &self.reference_accounts
  }

  pub fn exist_reference_account(&self, reference_account: usize) -> bool {
    self.reference_accounts.contains(&reference_account)
  }

  pub fn add_reference_account(&mut self, reference_account: usize) {
    self.reference_accounts.insert(reference_account);
    self.update_modify_time();
  }

  pub fn remove_reference_account(&mut self, reference_account: usize) {
    if self.reference_accounts.remove(&reference_account) == false {
      panic!("Reference account (id={reference_account}) does not exist");
    }
    self.update_modify_time();
  }

  pub fn clear_reference_accounts(&mut self) {
    self.reference_accounts.clear();
  }

  pub fn referenced_by_accounts(&self) -> &BTreeSet<usize> {
    &self.referenced_by_accounts
  }

  pub fn exist_referenced_by_account(&self, referenced_by_account: usize) -> bool {
    self.referenced_by_accounts.contains(&referenced_by_account)
  }

  pub fn add_referenced_by_account(&mut self, referenced_by_account: usize) {
    self.referenced_by_accounts.insert(referenced_by_account);
    self.update_modify_time();
  }

  pub fn remove_referenced_by_account(&mut self, referenced_by_account: usize) {
    if self.referenced_by_accounts.remove(&referenced_by_account) == false {
      panic!("Referenced by account (id={referenced_by_account}) does not exist");
    }
    self.update_modify_time();
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
    self.update_modify_time();
  }

  pub fn service(&self) -> Option<&String> {
    self.service.as_ref()
  }

  pub fn set_service(&mut self, service: Option<String>) {
    self.service = service;
    self.update_modify_time();
  }

  pub fn login_name(&self) -> Option<&String> {
    self.login_name.as_ref()
  }

  pub fn set_login_name(&mut self, login_name: Option<String>) {
    self.login_name = login_name;
    self.update_modify_time();
  }

  pub fn password(&self) -> Option<&Password> {
    self.password.as_ref()
  }

  pub fn password_mut(&mut self) -> Option<&mut Password> {
    self.password.as_mut()
  }

  pub fn set_password(&mut self, plain: Option<String>, secondary_password: &[u8], secondary_password_nonce: &[u8]) {
    self.password = match plain {
      Some(plain) => {
        Some(Password {
          ciphered: Password::cipher(plain, secondary_password, secondary_password_nonce),
          plain: None,
        })
      }
      None => {
        None
      }
    };
    self.update_modify_time();
  }

  pub fn comment(&self) -> Option<&String> {
    self.comment.as_ref()
  }

  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
    self.update_modify_time();
  }

  pub fn custom_fields(&self) -> &BTreeMap<String, String> {
    &self.custom_fields
  }

  pub fn add_custom_field(&mut self, key: String, value: String) {
    self.custom_fields.insert(key, value);
    self.update_modify_time();
  }

  pub fn remove_custom_field(&mut self, key: String) {
    self.custom_fields.remove(&key);
    self.update_modify_time();
  }

  pub fn clear_custom_fields(&mut self) {
    self.custom_fields.clear();
    self.update_modify_time();
  }

  pub fn create_time(&self) -> DateTime<Local> {
    self.create_time
  }

  pub fn modify_time(&self) -> DateTime<Local> {
    self.modify_time
  }

  fn update_modify_time(&mut self) {
    self.modify_time = Local::now()
  }

}

impl Account {

  pub fn from_reader(reader: &mut ByteSliceReader) -> Result<Account, Box<dyn Error>> {
    let id = reader.read_usize()?;
    let parent_account = reader.read_option_usize()?;
    let children_accounts = reader.read_btreeset_usize()?;
    let reference_accounts = reader.read_btreeset_usize()?;
    let referenced_by_accounts = reader.read_btreeset_usize()?;
    let name = reader.read_string()?;
    let service = reader.read_option_string()?;
    let login_name = reader.read_option_string()?;
    let password = match reader.read_option_vec_u8()? {
      Some(ciphered_password) => {
        Some(Password {
          ciphered: ciphered_password,
          plain: None,
        })
      }
      None => {
        None
      }
    };
    let comment = reader.read_option_string()?;
    let custom_fields = reader.read_btreemap_string_string()?;
    let create_time = reader.read_datetime_local()?;
    let modify_time = reader.read_datetime_local()?;

    Ok(Account {
      id,
      parent_account,
      children_accounts,
      reference_accounts,
      referenced_by_accounts,
      name,
      service,
      login_name,
      password,
      comment,
      custom_fields,
      create_time,
      modify_time,
    })
  }

  pub fn write(&self, writer: &mut ByteVecWriter) {
    writer.write_usize(self.id);
    writer.write_option_usize(self.parent_account);
    writer.write_btreeset_usize(&self.children_accounts);
    writer.write_btreeset_usize(&self.reference_accounts);
    writer.write_btreeset_usize(&self.referenced_by_accounts);
    writer.write_string(&self.name);
    writer.write_option_string(&self.service);
    writer.write_option_string(&self.login_name);
    writer.write_option_vec_u8(self.password.as_ref().map(|p| p.ciphered.as_slice()));
    writer.write_option_string(&self.comment);
    writer.write_btreemap_string_string(&self.custom_fields);
    writer.write_datetime_local(&self.create_time);
    writer.write_datetime_local(&self.modify_time);
  }

}
