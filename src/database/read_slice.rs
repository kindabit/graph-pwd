use std::error::Error;

/// Read itself from a byte slice
pub trait ReadSlice<'a>
{

  /// Returns itself and a new slice
  fn read_slice(slice: &'a [u8]) -> Result<(Self, &'a [u8]), Box<dyn Error>> where Self: Sized;

}
