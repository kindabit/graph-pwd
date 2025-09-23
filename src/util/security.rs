pub fn erase_string(mut s: String) {
  let repeat_zero = "0".repeat(s.len());
  s.clear();
  s.push_str(&repeat_zero);
}

pub fn erase_string_not_own(s: &mut String) {
  let repeat_zero = "0".repeat(s.len());
  s.clear();
  s.push_str(&repeat_zero);
}
