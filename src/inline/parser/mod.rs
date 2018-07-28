mod input;

pub use self::input::ParserInput;
use crate::inline::ast;

pub fn parse<'a>(input: ParserInput<'a>) -> ast::Inline<'a> {
  unimplemented!();
}
