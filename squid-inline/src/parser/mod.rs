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
    use crate::ast::{
        self, InlineEntity, InlineEntityNode, InlineFormatting, InlineFormattingNode,
    };
    use squid_core::span::Span;

    #[test]
    fn test_parse_plain_text_works() {
        let result = parse(ParserInputBuilder::new("foo bar baz").build());

        let expected = vec![InlineFormatting::new(
            Span::new(0, 11),
            ast::InlineFormattingNode::Normal(vec![ast::InlineEntity::new(
                Span::new(0, 11),
                ast::InlineEntityNode::Text("foo bar baz"),
            )]),
        )];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_plain_text_works_with_base_span() {
        let result = parse(
            ParserInputBuilder::new("foo bar baz")
                .with_base_span(Span::new(100, 0))
                .build(),
        );

        let expected = vec![InlineFormatting::new(
            Span::new(100, 11),
            InlineFormattingNode::Normal(vec![InlineEntity::new(
                Span::new(100, 11),
                InlineEntityNode::Text("foo bar baz"),
            )]),
        )];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_emphasis_works() {
        let result = parse(ParserInputBuilder::new("foo *bar* baz").build());
        let expected = vec![
            InlineFormatting::new(
                Span::new(0, 4),
                InlineFormattingNode::Normal(vec![InlineEntity::new(
                    Span::new(0, 4),
                    InlineEntityNode::Text("foo "),
                )]),
            ),
            InlineFormatting::new(
                Span::new(4, 5),
                InlineFormattingNode::Emphasis(vec![InlineEntity::new(
                    Span::new(5, 3),
                    InlineEntityNode::Text("bar"),
                )]),
            ),
            InlineFormatting::new(
                Span::new(5, 4),
                InlineFormattingNode::Normal(vec![InlineEntity::new(
                    Span::new(5, 4),
                    InlineEntityNode::Text("foo "),
                )]),
            ),
        ];

        assert_eq!(expected, result);
    }
}
