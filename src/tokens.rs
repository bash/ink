use std::borrow::Cow;
use super::ast::HeadingLevel;

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl LineType {
    pub fn is_heading(&self) -> bool {
        matches!(
            *self,
            LineType::Heading1 | LineType::Heading2 | LineType::Heading3
        )
    }

    pub fn get_heading_level(&self) -> Option<HeadingLevel> {
        match *self {
            LineType::Heading1 => Some(HeadingLevel::Level1),
            LineType::Heading2 => Some(HeadingLevel::Level2),
            LineType::Heading3 => Some(HeadingLevel::Level3),
            _ => None,
        }
    }
}
