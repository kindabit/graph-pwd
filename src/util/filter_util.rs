use crate::database::account::Account;

pub fn is_match(account: &Account, filter: &str) -> bool {
  if filter.len() == 0 {
    true
  }
  else if account.name().to_lowercase().contains(filter) {
    true
  }
  else if let Some(service) = account.service() && service.to_lowercase().contains(filter) {
    true
  }
  else if let Some(login_name) = account.login_name() && login_name.to_lowercase().contains(filter) {
    true
  }
  else {
    false
  }
}
