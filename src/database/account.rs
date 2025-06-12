use std::{collections::{HashMap, HashSet}, error::Error};

use chrono::{DateTime, Local};

use crate::util::{ByteSliceReader, ByteVecWriter};

#[derive(Debug)]
pub struct Account {

  id: usize,

  parent_account: Option<usize>,

  children_accounts: HashSet<usize>,

  reference_accounts: HashSet<usize>,

  referenced_by_accounts: HashSet<usize>,

  name: String,

  service: Option<String>,

  login_name: Option<String>,

  password: Option<String>,

  comment: Option<String>,

  custom_fields: HashMap<String, String>,

  create_time: DateTime<Local>,

  modify_time: DateTime<Local>,

}

impl Account {

  pub(super) fn new(id: usize, name: String, parent_account: Option<usize>) -> Self {
    let now = Local::now();
    Self {
      id,
      parent_account,
      children_accounts: HashSet::new(),
      reference_accounts: HashSet::new(),
      referenced_by_accounts: HashSet::new(),
      name,
      service: None,
      login_name: None,
      password: None,
      comment: None,
      custom_fields: HashMap::new(),
      create_time: now.clone(),
      modify_time: now.clone(),
    }
  }

  pub fn parent_account(&self) -> Option<usize> {
    self.parent_account
  }

  pub fn set_parent_account(&mut self, parent_account: Option<usize>) {
    self.parent_account = parent_account;
    self.update_modify_time();
  }

  pub fn exist_children_account(&self, children_account: usize) -> bool {
    self.children_accounts.contains(&children_account)
  }

  pub fn add_children_account(&mut self, children_account: usize) {
    self.children_accounts.insert(children_account);
    self.update_modify_time();
  }

  pub fn remove_children_account(&mut self, children_account: usize) {
    self.children_accounts.remove(&children_account);
    self.update_modify_time();
  }

  pub fn exist_reference_account(&self, reference_account: usize) -> bool {
    self.reference_accounts.contains(&reference_account)
  }

  pub fn add_reference_account(&mut self, reference_account: usize) {
    self.reference_accounts.insert(reference_account);
    self.update_modify_time();
  }

  pub fn remove_reference_account(&mut self, reference_account: usize) {
    self.reference_accounts.remove(&reference_account);
    self.update_modify_time();
  }

  pub fn exist_referenced_by_account(&self, referenced_by_account: usize) -> bool {
    self.referenced_by_accounts.contains(&referenced_by_account)
  }

  pub fn add_referenced_by_account(&mut self, referenced_by_account: usize) {
    self.referenced_by_accounts.insert(referenced_by_account);
    self.update_modify_time();
  }

  pub fn remove_referenced_by_account(&mut self, referenced_by_account: usize) {
    self.referenced_by_accounts.remove(&referenced_by_account);
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

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }

  pub fn set_password(&mut self, password: Option<String>) {
    self.password = password;
    self.update_modify_time();
  }

  pub fn comment(&self) -> Option<&String> {
    self.comment.as_ref()
  }

  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
    self.update_modify_time();
  }

  pub fn custom_fields(&self) -> &HashMap<String, String> {
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
    let children_accounts = reader.read_hashset_usize()?;
    let reference_accounts = reader.read_hashset_usize()?;
    let referenced_by_accounts = reader.read_hashset_usize()?;
    let name = reader.read_string()?;
    let service = reader.read_option_string()?;
    let login_name = reader.read_option_string()?;
    let password = reader.read_option_string()?;
    let comment = reader.read_option_string()?;
    let custom_fields = reader.read_hashmap_string_string()?;
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
    writer.write_hashset_usize(&self.children_accounts);
    writer.write_hashset_usize(&self.reference_accounts);
    writer.write_hashset_usize(&self.referenced_by_accounts);
    writer.write_string(&self.name);
    writer.write_option_string(&self.service);
    writer.write_option_string(&self.login_name);
    writer.write_option_string(&self.password);
    writer.write_option_string(&self.comment);
    writer.write_hashmap_string_string(&self.custom_fields);
    writer.write_datetime_local(&self.create_time);
    writer.write_datetime_local(&self.modify_time);
  }

}
