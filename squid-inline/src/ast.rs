use squid_core::span::Span;

pub type Inline<'a> = Vec<Formatting<'a>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FormattingType {
    Normal,
    Emphasis,
    StrongEmphasis,
    UltraEmphasis,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Formatting<'a> {
    span: Span,
    formatting: FormattingType,
    entities: Vec<Entity<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entity<'a> {
    span: Span,
    kind: EntityNode<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EntityNode<'a> {
    Text(&'a str),
    LineBreak,
    Link {
        label: Option<&'a str>,
        url: &'a str,
    },
}

impl<'a> Formatting<'a> {
    pub(crate) fn new(span: Span, formatting: FormattingType, entities: Vec<Entity<'a>>) -> Self {
        Formatting {
            span,
            formatting,
            entities,
        }
    }
}

impl<'a> Entity<'a> {
    pub(crate) fn new(span: Span, kind: EntityNode<'a>) -> Self {
        Entity { span, kind }
    }
}
