use super::text::Text;

pub type Document = Vec<Block>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    #[doc(hidden)]
    __NonExhaustive,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ListType {
    Unordered,
    Ordered,
}

#[derive(Debug, Eq, PartialEq)]
// TODO: needs to be properly defined
pub struct Decorator;

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Heading(HeadingLevel, String),
    Paragraph(Text),
    Quote(Text),
    FencedBlock(Option<Decorator>, String),
    List(ListType, Vec<Text>),
}