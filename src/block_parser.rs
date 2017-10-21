use super::block_tokenizer::{BlockTokenizer, LineType};

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
pub struct TextAccumulator {
    buffer: String,
}

impl BlockParser {
    pub fn new(tokenizer: BlockTokenizer) -> Self {
        BlockParser { tokenizer }
    }

    fn parse_text(&mut self) -> Option<Block> {
        let mut accumulator = TextAccumulator::new();

        loop {
            let next_type = self.tokenizer.peek();

            if let Some(LineType::Text) = next_type {
                let line = self.tokenizer.consume(next_type.unwrap()).unwrap();

                accumulator.add(&line.value().unwrap());
            } else {
                break;
            }
        }

        Some(Block::Text { content: accumulator.consume() })
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

impl TextAccumulator {
    pub fn new() -> Self {
        TextAccumulator { buffer: String::new() }
    }

    ///
    /// Adds a new line to the current accumulated text.
    /// Todo: this should also take care of newlines with two spaces
    ///
    pub fn add(&mut self, line: &str) {
        if self.buffer.len() > 0 {
            self.buffer.push_str(" ");
        }

        self.buffer.push_str(line);
    }

    pub fn consume(self) -> String {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::block_tokenizer::BlockTokenizer;

    #[test]
    fn it_works() {
        let mut parser = BlockParser::new(BlockTokenizer::new("Lorem ipsum\ndolor sit amet"));

        assert_eq!(
            Some(Block::Text {
                content: "Lorem ipsum dolor sit amet".to_string(),
            }),
            parser.next()
        );
    }
}