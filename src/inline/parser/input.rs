use crate::span::Span;

#[derive(Debug)]
pub struct ParserInput<'a> {
  input: &'a str,
  pos: usize,
  base_span: Span,
}

impl<'a> ParserInput<'a> {
  pub fn new(input: &'a str, base_span: Option<Span>) -> Self {
    ParserInput {
      input,
      pos: 0,
      base_span: base_span.unwrap_or_else(|| Span::new(0, input.len())),
    }
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
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_starts_with_works() {
    let mut input = ParserInput::new("foo *bar* baz", None);

    assert_eq!(false, input.starts_with("*"));

    input.skip_chars(4);

    assert_eq!(true, input.starts_with("*"));
  }
}
