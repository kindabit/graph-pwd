use crate::database::{account::Account, Database};

pub fn traverse_account_tree(
  current_account_id: usize,
  database: &Database,
  op: &mut impl FnMut(&Account) -> ()
) {
  let current_account = database.accounts()
    .get(current_account_id).expect(&format!("Account id ({current_account_id}) out of range"))
    .as_ref().expect(&format!("Account (id={current_account_id}) has already been deleted"));
  op(current_account);
  for child_account_id in current_account.children_accounts() {
    traverse_account_tree(*child_account_id, database, op);
  }
}
