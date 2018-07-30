use char_slice::CharSlice;
use squid_core::span::Span;

const EMPHASIS_TOKEN: (usize, &str) = (1, "*");
const STRONG_EMPHASIS_TOKEN: (usize, &str) = (2, "**");
const ULTRA_EMPHASIS_TOKEN: (usize, &str) = (3, "***");
const LINK_LEFT_TOKEN: &str = "<";
const LINK_RIGHT_TOKEN: &str = ">";

#[derive(Debug)]
pub struct ParserInputBuilder<'a> {
    input: &'a str,
    base_span: Option<Span>,
}

#[derive(Debug)]
pub struct ParserInput<'a> {
    input: &'a str,
    /// Current position in chars (not bytes)
    pos: usize,
    base_span: Span,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ParserToken {
    left: bool,
    right: bool,
    kind: ParserTokenInner,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ParserTokenInner {
    Emphasis,
    StrongEmphasis,
    UltraEmphasis,
    Link,
}

fn is_empty_or_whitespace(slice: &str) -> bool {
    slice.is_empty() || slice.chars().all(char::is_whitespace)
}

impl ParserToken {
    pub(crate) fn left(&self) -> bool {
        self.left
    }

    pub(crate) fn right(&self) -> bool {
        self.right
    }

    pub(crate) fn kind(&self) -> ParserTokenInner {
        self.kind
    }
}

impl<'a> ParserInputBuilder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            base_span: None,
        }
    }

    pub fn with_base_span(mut self, base_span: Span) -> Self {
        self.base_span = base_span.into();
        self
    }

    pub fn build(self) -> ParserInput<'a> {
        ParserInput {
            input: self.input,
            pos: 0,
            base_span: self
                .base_span
                .unwrap_or_else(|| Span::new(0, self.input.len())),
        }
    }
}

impl<'a> ParserInput<'a> {
    pub(crate) fn take(&mut self, chars: usize) -> (Span, &'a str) {
        let consumed = &self.input.char_slice(self.pos..(self.pos + chars));
        let span = self.create_span(chars);

        self.pos += chars;

        (span, consumed)
    }

    pub(crate) fn len(&self) -> usize {
        self.input.char_slice(self.pos..).chars().count()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.input.char_slice(self.pos..).is_empty()
    }

    pub(crate) fn skip_chars(&mut self, chars: usize) {
        self.pos += chars;
    }

    pub(crate) fn starts_with(&self, needle: &'a str) -> bool {
        self.input[self.pos..].starts_with(needle)
    }

    pub(crate) fn next_token(&self) -> Option<ParserToken> {
        macro expect_token($token_matcher:expr, $token:expr) {
            let token_chars = $token_matcher.0;
            let next_char_pos = self.pos + token_chars;

            if self.starts_with($token_matcher.1) {
                return Some(ParserToken {
                    left: !is_empty_or_whitespace(
                        &self.input.char_slice(next_char_pos..(next_char_pos + 1)),
                    ),
                    right: !is_empty_or_whitespace(self.previous_char()),
                    kind: $token,
                });
            }
        }

        expect_token!(ULTRA_EMPHASIS_TOKEN, ParserTokenInner::UltraEmphasis);
        expect_token!(STRONG_EMPHASIS_TOKEN, ParserTokenInner::StrongEmphasis);
        expect_token!(EMPHASIS_TOKEN, ParserTokenInner::Emphasis);

        if self.starts_with(LINK_LEFT_TOKEN) {
            return Some(ParserToken {
                left: true,
                right: false,
                kind: ParserTokenInner::Link,
            });
        }

        if self.starts_with(LINK_RIGHT_TOKEN) {
            return Some(ParserToken {
                left: false,
                right: true,
                kind: ParserTokenInner::Link,
            });
        }

        None
    }

    fn previous_char(&self) -> &str {
        let start = if self.pos == 0 {
            self.pos
        } else {
            self.pos - 1
        };

        self.input.char_slice(start..self.pos)
    }

    fn create_span(&self, len: usize) -> Span {
        Span::with_base(self.base_span, self.pos, len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_starts_with_works() {
        let mut input = ParserInputBuilder::new("foo *bar* baz").build();

        assert_eq!(false, input.starts_with("*"));

        input.skip_chars(4);

        assert_eq!(true, input.starts_with("*"));
    }

    #[test]
    fn test_take_works() {
        let mut input = ParserInputBuilder::new("føø bar baz")
            .with_base_span(Span::new(200, 0))
            .build();

        assert_eq!((Span::new(200, 3), "føø"), input.take(3));
        assert_eq!((Span::new(203, 4), " bar"), input.take(4));
        assert_eq!((Span::new(207, 4), " baz"), input.take(4));
    }

    #[test]
    fn test_next_token_works() {
        let mut input = ParserInputBuilder::new("*foo*bar*").build();

        assert_eq!(
            ParserToken {
                left: true,
                right: false,
                kind: ParserTokenInner::Emphasis,
            },
            input.next_token().unwrap()
        );

        input.take(4);

        assert_eq!(
            ParserToken {
                left: true,
                right: true,
                kind: ParserTokenInner::Emphasis,
            },
            input.next_token().unwrap()
        );

        input.take(4);

        assert_eq!(
            ParserToken {
                left: false,
                right: true,
                kind: ParserTokenInner::Emphasis,
            },
            input.next_token().unwrap()
        );
    }

    #[test]
    fn test_strong_emphasis_token_works() {
        let mut input = ParserInputBuilder::new("**foo**bar**").build();

        assert_eq!(
            ParserToken {
                left: true,
                right: false,
                kind: ParserTokenInner::StrongEmphasis,
            },
            input.next_token().unwrap()
        );

        input.take(5);

        assert_eq!(
            ParserToken {
                left: true,
                right: true,
                kind: ParserTokenInner::StrongEmphasis,
            },
            input.next_token().unwrap()
        );

        input.take(5);

        assert_eq!(
            ParserToken {
                left: false,
                right: true,
                kind: ParserTokenInner::StrongEmphasis,
            },
            input.next_token().unwrap()
        );
    }

    #[test]
    fn test_ultra_emphasis_token_works() {
        let mut input = ParserInputBuilder::new("***foo***bar***").build();

        assert_eq!(
            ParserToken {
                left: true,
                right: false,
                kind: ParserTokenInner::UltraEmphasis,
            },
            input.next_token().unwrap()
        );

        input.take(6);

        assert_eq!(
            ParserToken {
                left: true,
                right: true,
                kind: ParserTokenInner::UltraEmphasis,
            },
            input.next_token().unwrap()
        );

        input.take(6);

        assert_eq!(
            ParserToken {
                left: false,
                right: true,
                kind: ParserTokenInner::UltraEmphasis,
            },
            input.next_token().unwrap()
        );
    }

    #[test]
    fn test_previous_char_works() {
        let mut input = ParserInputBuilder::new("foo").build();

        assert_eq!("", input.previous_char());
        input.take(1);
        assert_eq!("f", input.previous_char());
        input.take(1);
        assert_eq!("o", input.previous_char());
        input.take(1);
        assert_eq!("o", input.previous_char());
    }

    #[test]
    fn test_link_token_works() {
        assert_eq!(
            Some(ParserToken {
                left: true,
                right: false,
                kind: ParserTokenInner::Link,
            }),
            ParserInputBuilder::new("<").build().next_token()
        );

        assert_eq!(
            Some(ParserToken {
                left: false,
                right: true,
                kind: ParserTokenInner::Link,
            }),
            ParserInputBuilder::new(">").build().next_token()
        );
    }
}
