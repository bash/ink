use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub enum Line<'a> {
    Blank,
    Divider,
    Heading1(Cow<'a, str>),
    Heading2(Cow<'a, str>),
    Heading3(Cow<'a, str>),
    Text(Cow<'a, str>),
    Quote(Cow<'a, str>),
    Decorator(Cow<'a, str>),
    UnorderedList(Cow<'a, str>),
    OrderedList(Cow<'a, str>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LineType {
    Blank,
    Divider,
    Heading1,
    Heading2,
    Heading3,
    Text,
    Quote,
    Decorator,
    UnorderedList,
    OrderedList,
}

impl<'a> Line<'a> {
    pub fn value(self) -> Option<Cow<'a, str>> {
        match self {
            Line::Blank | Line::Divider => None,
            Line::Heading1(value) |
            Line::Heading2(value) |
            Line::Heading3(value) |
            Line::Text(value) |
            Line::Quote(value) |
            Line::Decorator(value) |
            Line::UnorderedList(value) |
            Line::OrderedList(value) => Some(value),
        }
    }
}
