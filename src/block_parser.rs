use super::block_tokenizer::{BlockTokenizer, Line};

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

#[derive(Debug)]
pub enum Block {
    Heading {
        level: HeadingLevel,
        content: String,
    },
    Text { content: String },
    FencedBlock {
        decorator: Option<String>,
        content: String,
    },
    List {
        list_type: ListType,
        items: Vec<String>,
    },
}

#[derive(Debug)]
pub struct BlockParser {
    tokenizer: BlockTokenizer,
    current: Option<Block>,
}

impl Iterator for BlockParser {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokenizer.consume_line() {
            Some(Line::Heading1(content)) => Some(Block::Heading {
                level: HeadingLevel::Level1,
                content,
            }),
            _ => None,
        }
    }
}