mod input;

pub use self::input::{ParserInput, ParserInputBuilder};
use crate::ast::{Entity, EntityNode, Formatting, FormattingType, Inline};

pub fn parse<'a>(mut input: ParserInput<'a>) -> Inline<'a> {
    let len = input.len();
    let (span, text) = input.take(len);

    let entities = parse_entities(ParserInputBuilder::new(text).with_base_span(span).build());

    vec![Formatting::new(span, FormattingType::Normal, entities)]
}

fn parse_entities(mut input: ParserInput<'a>) -> Vec<Entity<'a>> {
    let len = input.len();
    let (span, text) = input.take(len);

    vec![Entity::new(span, EntityNode::Text(text))]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Entity, EntityNode, Formatting, FormattingType};
    use squid_core::span::Span;

    #[test]
    fn test_parse_plain_text_works() {
        let result = parse(ParserInputBuilder::new("foo bar baz").build());

        let expected = vec![Formatting::new(
            Span::new(0, 11),
            FormattingType::Normal,
            vec![Entity::new(
                Span::new(0, 11),
                EntityNode::Text("foo bar baz"),
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

        let expected = vec![Formatting::new(
            Span::new(100, 11),
            FormattingType::Normal,
            vec![Entity::new(
                Span::new(100, 11),
                EntityNode::Text("foo bar baz"),
            )],
        )];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_emphasis_works() {
        let result = parse(ParserInputBuilder::new("foo *bar* baz").build());
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
