static CHARS: [char; 94] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=',
  '[', '{', ']', '}', '|', '\\', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?'
];

pub fn usize_to_string_94_12(mut n: usize) -> String {
  let mut s = String::with_capacity(12);
  loop {
    s.push(CHARS[n % 94]);
    n /= 94;
    if n == 0 {
      break;
    }
  }
  while s.len() < 12 {
    s.push('0');
  }
  if s.len() != 12 {
    panic!("String's length is not 12 ({s})");
  }
  s
}

pub fn string_to_usize_94_12(s: &str) -> usize {
  if s.len() != 12 {
    panic!("String's length is not 12 ({s})");
  }

  let mut chars: [char; 12] = ['0'; 12];
  s.chars().enumerate().for_each(|(index, ch)| {
    chars[index] = ch;
  });
  let mut last_significant_idx = 12_usize;
  loop {
    last_significant_idx -= 1;
    if chars[last_significant_idx] != '0' {
      break;
    }
    else if last_significant_idx == 0 {
      break;
    }
  }
  let end = last_significant_idx + 1;
  chars[0..end].reverse();
  let mut n = 0_usize;
  for i in 0..end {
    let ch = chars[i];
    match ch {
      '0' => n += 0,
      '1' => n += 1,
      '2' => n += 2,
      '3' => n += 3,
      '4' => n += 4,
      '5' => n += 5,
      '6' => n += 6,
      '7' => n += 7,
      '8' => n += 8,
      '9' => n += 9,
      'a' => n += 10,
      'b' => n += 11,
      'c' => n += 12,
      'd' => n += 13,
      'e' => n += 14,
      'f' => n += 15,
      'g' => n += 16,
      'h' => n += 17,
      'i' => n += 18,
      'j' => n += 19,
      'k' => n += 20,
      'l' => n += 21,
      'm' => n += 22,
      'n' => n += 23,
      'o' => n += 24,
      'p' => n += 25,
      'q' => n += 26,
      'r' => n += 27,
      's' => n += 28,
      't' => n += 29,
      'u' => n += 30,
      'v' => n += 31,
      'w' => n += 32,
      'x' => n += 33,
      'y' => n += 34,
      'z' => n += 35,
      'A' => n += 36,
      'B' => n += 37,
      'C' => n += 38,
      'D' => n += 39,
      'E' => n += 40,
      'F' => n += 41,
      'G' => n += 42,
      'H' => n += 43,
      'I' => n += 44,
      'J' => n += 45,
      'K' => n += 46,
      'L' => n += 47,
      'M' => n += 48,
      'N' => n += 49,
      'O' => n += 50,
      'P' => n += 51,
      'Q' => n += 52,
      'R' => n += 53,
      'S' => n += 54,
      'T' => n += 55,
      'U' => n += 56,
      'V' => n += 57,
      'W' => n += 58,
      'X' => n += 59,
      'Y' => n += 60,
      'Z' => n += 61,
      '`' => n += 62,
      '~' => n += 63,
      '!' => n += 64,
      '@' => n += 65,
      '#' => n += 66,
      '$' => n += 67,
      '%' => n += 68,
      '^' => n += 69,
      '&' => n += 70,
      '*' => n += 71,
      '(' => n += 72,
      ')' => n += 73,
      '-' => n += 74,
      '_' => n += 75,
      '+' => n += 76,
      '=' => n += 77,
      '[' => n += 78,
      '{' => n += 79,
      ']' => n += 80,
      '}' => n += 81,
      '|' => n += 82,
      '\\' => n += 83,
      ';' => n += 84,
      ':' => n += 85,
      '\'' => n += 86,
      '"' => n += 87,
      ',' => n += 88,
      '<' => n += 89,
      '.' => n += 90,
      '>' => n += 91,
      '/' => n += 92,
      '?' => n += 93,
      other => {
        let mut buf: [u8; 4] = [0; 4];
        other.encode_utf8(&mut buf);
        panic!("Unexpected character: {buf:?}")
      }
    }
    if i < end - 1 {
      n *= 94;
    }
  }
  n
}
