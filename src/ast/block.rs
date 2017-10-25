#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ListType {
    Unordered,
    Ordered,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Heading {
        level: HeadingLevel,
        content: String,
    },
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