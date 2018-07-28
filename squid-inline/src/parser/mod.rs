mod input;

pub use self::input::{ParserInput, ParserInputBuilder};
use crate::ast::{FormattingType, Inline, InlineEntity, InlineEntityNode, InlineFormatting};

pub fn parse<'a>(mut input: ParserInput<'a>) -> Inline<'a> {
    let len = input.len();
    let (span, text) = input.take(len);

    let entities = parse_entities(ParserInputBuilder::new(text).with_base_span(span).build());

    vec![InlineFormatting::new(
        span,
        FormattingType::Normal,
        entities,
    )]
}

fn parse_entities(mut input: ParserInput<'a>) -> Vec<InlineEntity<'a>> {
    let len = input.len();
    let (span, text) = input.take(len);

    vec![InlineEntity::new(span, InlineEntityNode::Text(text))]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{self, FormattingType, InlineEntity, InlineEntityNode, InlineFormatting};
    use squid_core::span::Span;

    #[test]
    fn test_parse_plain_text_works() {
        let result = parse(ParserInputBuilder::new("foo bar baz").build());

        let expected = vec![InlineFormatting::new(
            Span::new(0, 11),
            FormattingType::Normal,
            vec![ast::InlineEntity::new(
                Span::new(0, 11),
                ast::InlineEntityNode::Text("foo bar baz"),
            )],
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
            FormattingType::Normal,
            vec![InlineEntity::new(
                Span::new(100, 11),
                InlineEntityNode::Text("foo bar baz"),
            )],
        )];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_emphasis_works() {
        let result = parse(ParserInputBuilder::new("foo *bar* baz").build());
        let expected = vec![
            InlineFormatting::new(
                Span::new(0, 4),
                FormattingType::Normal,
                vec![InlineEntity::new(
                    Span::new(0, 4),
                    InlineEntityNode::Text("foo "),
                )],
            ),
            InlineFormatting::new(
                Span::new(4, 5),
                FormattingType::Emphasis,
                vec![InlineEntity::new(
                    Span::new(5, 3),
                    InlineEntityNode::Text("bar"),
                )],
            ),
            InlineFormatting::new(
                Span::new(5, 4),
                FormattingType::Normal,
                vec![InlineEntity::new(
                    Span::new(5, 4),
                    InlineEntityNode::Text("foo "),
                )],
            ),
        ];

        assert_eq!(expected, result);
    }
}
