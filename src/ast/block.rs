use super::text::Text;

pub type Document = Vec<Block>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum ListType {
    Unordered,
    Ordered,
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Decorator {
    /// Decorator for a table block
    Table,
    /// Decorator for a code block. Contains the language name.
    Code(Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Heading(HeadingLevel, String),
    Paragraph(Text),
    Quote(Text),
    Preformatted(Option<Decorator>, String),
    List(ListType, Vec<Text>),
}
