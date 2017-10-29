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
            Line::Blank => None,
            Line::Divider => None,
            Line::Heading1(value) => Some(value),
            Line::Heading2(value) => Some(value),
            Line::Heading3(value) => Some(value),
            Line::Text(value) => Some(value),
            Line::Quote(value) => Some(value),
            Line::Decorator(value) => Some(value),
            Line::UnorderedList(value) => Some(value),
            Line::OrderedList(value) => Some(value),
        }
    }
}