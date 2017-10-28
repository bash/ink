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
pub struct Heading {
    level: HeadingLevel,
    content: String,
}

pub trait BlockInner {
    fn wrap(self) -> Block;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Heading(Heading),
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

impl Block {
    pub fn from_inner<I>(inner: I) -> Self
    where
        I: BlockInner,
    {
        inner.wrap()
    }
}

impl BlockInner for Heading {
    fn wrap(self) -> Block {
        Block::Heading(self)
    }
}

impl Heading {
    pub fn new(level: HeadingLevel, content: String) -> Self {
        Heading { level, content }
    }

    pub fn level(&self) -> HeadingLevel {
        self.level
    }

    // TODO: change to &'a str
    pub fn content(self) -> String {
        self.content
    }
}