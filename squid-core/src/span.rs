///
/// Represents a region of source code, used for error reporting and
/// source map generation.
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Span {
    /// Offset in bytes
    pub offset: usize,
    /// Length in bytes
    pub len: usize,
}

impl Span {
    ///
    /// Creates a new `Span` with a given offset and length.
    ///
    pub fn new(offset: usize, len: usize) -> Self {
        Span { offset, len }
    }

    ///
    /// Creates a new `Span` with a given base `Span`.
    ///
    pub fn with_base(base: Self, offset: usize, len: usize) -> Self {
        Span {
            offset: base.offset + offset,
            len: len,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_base() {
        let result = Span::with_base(Span::new(100, 0), 24, 6);

        assert_eq!(6, result.len);
        assert_eq!(124, result.offset);
    }
}
