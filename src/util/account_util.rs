use crate::database::Database;

pub fn get_account_short_form(id: usize, database: &Database) -> String {
  format!(
    "{}. {}",
    id,
    database.accounts().get(id)
      .expect(&format!("Account id {id} out of bounds"))
      .as_ref()
      .expect(&format!("Account (id={id}) is deleted"))
      .name(),
  )
}

pub fn is_weak_password(password: &str) -> bool {
  if password.len() < 8 {
    true
  }
  else {
    false
  }
}
