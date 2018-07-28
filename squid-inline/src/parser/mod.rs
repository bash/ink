mod input;

pub use self::input::{ParserInput, ParserInputBuilder};
use crate::ast;

pub fn parse<'a>(mut input: ParserInput<'a>) -> ast::Inline<'a> {
    let len = input.len();
    let (span, text) = input.take(len);

    vec![ast::InlineFormatting {
        span,
        kind: ast::InlineFormattingNode::Normal(vec![ast::InlineEntity {
            span,
            kind: ast::InlineEntityNode::Text(text),
        }]),
    }]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast;
    use squid_core::span::Span;

    #[test]
    fn test_parse_plain_text_works() {
        let result = parse(ParserInputBuilder::new("foo bar baz").build());

        let expected = vec![ast::InlineFormatting {
            span: Span::new(0, 11),
            kind: ast::InlineFormattingNode::Normal(vec![ast::InlineEntity {
                span: Span::new(0, 11),
                kind: ast::InlineEntityNode::Text("foo bar baz"),
            }]),
        }];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_plain_text_works_with_base_span() {
        let result = parse(
            ParserInputBuilder::new("foo bar baz")
                .with_base_span(Span::new(100, 0))
                .build(),
        );

        let expected = vec![ast::InlineFormatting {
            span: Span::new(100, 11),
            kind: ast::InlineFormattingNode::Normal(vec![ast::InlineEntity {
                span: Span::new(100, 11),
                kind: ast::InlineEntityNode::Text("foo bar baz"),
            }]),
        }];

        assert_eq!(result, expected);
    }
}
