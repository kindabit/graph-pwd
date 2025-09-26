use std::error::Error;

/// Append itself to the end of a buffer
pub trait AppendBuffer
 {

  /// Returns the number of bytes which were appended to the end of the buffer
  fn append_buffer(&self, buffer: &mut Vec<u8>) -> Result<usize, Box<dyn Error>>;

}
