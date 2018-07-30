///
/// Represents a region of source code, used for error reporting and
/// source map generation.
///
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
pub struct Span {
    /// Offset in chars
    offset: usize,
    /// Length in chars
    len: usize,
}

impl Span {
    ///
    /// Creates a new `Span` with a given character offset and length.
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

    ///
    /// The character offset of this `Span`
    ///
    pub fn offset(&self) -> usize {
        self.offset
    }

    ///
    /// The length of this `Span` in characters.
    ///
    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "use_serde")]
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_with_base() {
        let result = Span::with_base(Span::new(100, 0), 24, 6);

        assert_eq!(6, result.len());
        assert_eq!(124, result.offset());
    }

    #[test]
    #[cfg(feature = "use_serde")]
    fn test_ser_de() {
        let span = Span::new(1, 2);

        assert_tokens(
            &span,
            &[
                Token::Struct {
                    name: "Span",
                    len: 2,
                },
                Token::String("offset"),
                Token::U64(1),
                Token::String("len"),
                Token::U64(2),
                Token::StructEnd,
            ],
        )
    }
}
