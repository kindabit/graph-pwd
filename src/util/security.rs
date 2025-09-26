use std::error::Error;

use aes_gcm_siv::{aead::Aead, Aes256GcmSiv, KeyInit, Nonce};

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

fn create_aes(key: &[u8]) -> Result<Aes256GcmSiv, Box<dyn Error>> {
  match Aes256GcmSiv::new_from_slice(key) {
    Ok(instance) => {
      Ok(instance)
    }
    Err(err) => {
      Err(Box::new(err))
    }
  }
}

pub fn encrypt(plain: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  let aes = create_aes(key)?;

  match aes.encrypt(Nonce::from_slice(nonce), plain) {
    Ok(ciphered) => {
      Ok(ciphered)
    }
    Err(err) => {
      Err(Box::new(err))
    }
  }
}

pub fn decrypt(ciphered: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  let cipher = create_aes(key)?;

  match cipher.decrypt(Nonce::from_slice(nonce), ciphered) {
    Ok(plain) => {
      Ok(plain)
    }
    Err(err) => {
      Err(Box::new(err))
    }
  }
}
