use squid_core::span::Span;

#[derive(Debug)]
pub struct ParserInputBuilder<'a> {
    input: &'a str,
    base_span: Option<Span>,
}

#[derive(Debug)]
pub struct ParserInput<'a> {
    input: &'a str,
    pos: usize,
    base_span: Span,
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
    pub(crate) fn take(&mut self, chars: usize) -> &'a str {
        let len = self.input[self.pos..]
            .chars()
            .take(chars)
            .fold(0, |acc, ch| acc + ch.len_utf8());

        let consumed = &self.input[self.pos..(self.pos + len)];

        self.pos += len;

        consumed
    }

    pub(crate) fn len(&self) -> usize {
        self.input[self.pos..].len()
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn skip_chars(&mut self, chars: usize) {
        let len = self.input[self.pos..]
            .chars()
            .take(chars)
            .fold(0, |acc, ch| acc + ch.len_utf8());

        self.pos += len;
    }

    pub(crate) fn starts_with(&self, needle: &'a str) -> bool {
        self.input[self.pos..].starts_with(needle)
    }

    pub(crate) fn create_span(&self, offset: usize, len: usize) -> Span {
        Span::with_base(self.base_span, offset, len)
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
}
