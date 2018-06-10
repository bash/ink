///
/// Represents a region of code, used for error reporting and
/// source map generation.
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Span {
  pub offset: usize,
  pub len: usize,
}

impl Span {
  pub fn new(offset: usize, len: usize) -> Self {
    Span { offset, len }
  }

  ///
  /// Turns a relative span into an absolute `Span`
  /// given a base `Span`.
  ///
  pub fn absolute(&self, other: Self) -> Self {
    Span {
      offset: other.offset + self.offset,
      len: self.len,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_absolute() {
    let result = Span::new(24, 6).absolute(Span::new(100, 0));

    assert_eq!(6, result.len);
    assert_eq!(124, result.offset);
  }
}
