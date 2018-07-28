use squid_core::span::Span;

pub type Inline<'a> = Vec<InlineFormatting<'a>>;

#[derive(Debug, Eq, PartialEq)]
pub struct InlineFormatting<'a> {
    pub span: Span,
    pub kind: InlineFormattingNode<'a>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InlineFormattingNode<'a> {
    Normal(Vec<InlineEntity<'a>>),
    Emphasis(Vec<InlineEntity<'a>>),
    StrongEmphasis(Vec<InlineEntity<'a>>),
    UltraEmphasis(Vec<InlineEntity<'a>>),
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
    pub(crate) fn new(span: Span, kind: InlineFormattingNode<'a>) -> Self {
        Self { span, kind }
    }
}

impl<'a> InlineEntity<'a> {
    pub(crate) fn new(span: Span, kind: InlineEntityNode<'a>) -> Self {
        Self { span, kind }
    }
}
