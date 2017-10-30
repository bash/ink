use std::borrow::Cow;

pub type Document<'a> = Vec<Block<'a>>;

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
pub struct Heading<'a> {
    level: HeadingLevel,
    content: Cow<'a, str>,
}

pub trait BlockInner<'a> {
    fn wrap(self) -> Block<'a>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block<'a> {
    Heading(Heading<'a>),
    Text { content: String },
    Quote { content: String },
    FencedBlock {
        decorator: Option<String>,
        content: String,
    },
    List {
        list_type: ListType,
        items: Vec<String>,
    },
}

impl<'a> Block<'a> {
    pub fn from_inner<I>(inner: I) -> Self
    where
        I: BlockInner<'a>,
    {
        inner.wrap()
    }
}

impl<'a> BlockInner<'a> for Heading<'a> {
    fn wrap(self) -> Block<'a> {
        Block::Heading(self)
    }
}

impl<'a> Heading<'a> {
    pub fn new<S>(level: HeadingLevel, content: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Heading {
            level,
            content: content.into(),
        }
    }

    pub fn level(&self) -> HeadingLevel {
        self.level
    }

    pub fn content(&self) -> &Cow<'a, str> {
        &self.content
    }

    pub fn consume(self) -> (HeadingLevel, Cow<'a, str>) {
        (self.level, self.content)
    }
}