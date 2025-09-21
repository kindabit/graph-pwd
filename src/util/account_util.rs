use crate::database::{account::Account, Database};

pub fn get_account_short_form(id: usize, database: &Database) -> String {
  format!(
    "{}. {}",
    id,
    guarantee_account_from_database(id, database).name(),
  )
}

pub fn guarantee_account_from_database(id: usize, database: &Database) -> &Account {
  database
  .accounts()
  .get(id).expect(&format!("Account id {id} out of bounds"))
  .as_ref().expect(&format!("Account (id={id}) has already been deleted"))
}

pub fn is_weak_password(password: &str) -> bool {
  if password.len() < 8 {
    true
  }
  else {
    false
  }
}
