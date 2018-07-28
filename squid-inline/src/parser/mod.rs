mod input;

pub use self::input::ParserInput;
use crate::ast;

pub fn parse(_input: ParserInput<'_>) -> ast::Inline<'_> {
  unimplemented!();
}
