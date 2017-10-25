#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Blank,
    Divider,
    Heading1(String),
    Heading2(String),
    Heading3(String),
    Text(String),
    Quote(String),
    Decorator(String),
    UnorderedList(String),
    OrderedList(String),
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