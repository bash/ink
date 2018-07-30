mod input;

pub use self::input::{ParserInput, ParserInputBuilder};
use crate::ast::Inline;

#[derive(Debug)]
///
/// The inline parser takes a string and turns it into AST.
///
/// **Example**
///
/// ```rust
///  use squid_inline::parser::{Parser, ParserInputBuilder};
///
///  let parser = Parser::new(ParserInputBuilder::new("foo *bar* baz").build());
///  let ast = parser.parse();
/// ```
///
pub struct Parser<'a> {
    input: ParserInput<'a>,
}

impl<'a> Parser<'a> {
    ///
    /// Creates a new `Parser` with a given input.
    ///
    pub fn new(input: ParserInput<'a>) -> Self {
        Self { input }
    }

    ///
    /// Parses the parser's input into ast. Consumes the parser
    ///
    pub fn parse(self) -> Inline<'a> {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Entity, EntityNode, Formatting, FormattingType};
    use squid_core::span::Span;

    #[test]
    fn test_parse_plain_text_works() {
        let parser = Parser::new(ParserInputBuilder::new("foo bar baz").build());
        let result = parser.parse();

        let expected = vec![Formatting::new(
            Span::new(0, 11),
            FormattingType::Normal,
            vec![Entity::new(
                Span::new(0, 11),
                EntityNode::Text("foo bar bäz"),
            )],
        )];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_plain_text_works_with_base_span() {
        let parser = Parser::new(
            ParserInputBuilder::new("foo bar baz")
                .with_base_span(Span::new(100, 0))
                .build(),
        );
        let result = parser.parse();

        let expected = vec![Formatting::new(
            Span::new(100, 11),
            FormattingType::Normal,
            vec![Entity::new(
                Span::new(100, 11),
                EntityNode::Text("foo bar baz"),
            )],
        )];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_emphasis_works() {
        let parser = Parser::new(ParserInputBuilder::new("foo *bar* baz").build());
        let result = parser.parse();
        let expected = vec![
            Formatting::new(
                Span::new(0, 4),
                FormattingType::Normal,
                vec![Entity::new(Span::new(0, 4), EntityNode::Text("foo "))],
            ),
            Formatting::new(
                Span::new(4, 5),
                FormattingType::Emphasis,
                vec![Entity::new(Span::new(5, 3), EntityNode::Text("bar"))],
            ),
            Formatting::new(
                Span::new(5, 4),
                FormattingType::Normal,
                vec![Entity::new(Span::new(5, 4), EntityNode::Text("foo "))],
            ),
        ];

        assert_eq!(expected, result);
    }
}
