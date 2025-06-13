pub struct GlobalState {

  tree_mode: bool,

}

impl GlobalState {

  pub fn new() -> Self {
    Self {
      tree_mode: false,
    }
  }

  pub fn tree_mode(&self) -> bool {
    self.tree_mode
  }

  pub fn set_tree_mode(&mut self, tree_mode: bool) {
    self.tree_mode = tree_mode;
  }

}
