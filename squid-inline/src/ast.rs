use squid_core::span::Span;

pub type Inline<'a> = Vec<InlineFormatting<'a>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FormattingType {
    Normal,
    Emphasis,
    StrongEmphasis,
    UltraEmphasis,
}

#[derive(Debug, Eq, PartialEq)]
pub struct InlineFormatting<'a> {
    pub span: Span,
    pub formatting: FormattingType,
    pub entities: Vec<InlineEntity<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct InlineEntity<'a> {
    pub span: Span,
    pub kind: InlineEntityNode<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InlineEntityNode<'a> {
    Text(&'a str),
    LineBreak,
    Link {
        label: Option<&'a str>,
        url: &'a str,
    },
}

impl<'a> InlineFormatting<'a> {
    pub(crate) fn new(
        span: Span,
        formatting: FormattingType,
        entities: Vec<InlineEntity<'a>>,
    ) -> Self {
        Self {
            span,
            formatting,
            entities,
        }
    }
}

impl<'a> InlineEntity<'a> {
    pub(crate) fn new(span: Span, kind: InlineEntityNode<'a>) -> Self {
        Self { span, kind }
    }
}
