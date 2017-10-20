use super::block_tokenizer::{BlockTokenizer, Line, LineType};

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
}

#[derive(Debug)]
pub struct TextAccumulator {}

impl BlockParser {
    pub fn new(tokenizer: BlockTokenizer) -> Self {
        BlockParser { tokenizer }
    }

    fn parse_text(&mut self) -> Option<Block> {
        None
    }
}

impl Iterator for BlockParser {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.peek() {
                None => return None,
                Some(LineType::Blank) => continue,
                Some(LineType::Text) => return self.parse_text(),
                _ => return None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::block_tokenizer::BlockTokenizer;

    #[test]
    fn it_works() {
        let mut parser = BlockParser::new(BlockTokenizer::new("foo bar"));

        assert_eq!(Some(Block::Text { content: "foo bar".to_string() }), parser.next());
    }
}