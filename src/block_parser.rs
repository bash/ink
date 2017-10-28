use super::block_tokenizer::BlockTokenizer;
use super::tokens::LineType;
use super::ast::{Block, HeadingLevel};

#[derive(Debug)]
pub struct BlockParser {
    tokenizer: BlockTokenizer,
}

#[derive(Debug)]
pub struct TextAccumulator {
    buffer: String,
}

impl BlockParser {
    pub fn new<S: Into<String>>(input: S) -> Self {
        BlockParser { tokenizer: BlockTokenizer::new(input) }
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

    fn parse_quote(&mut self) -> Option<Block> {
        let mut accumulator = TextAccumulator::new();

        loop {
            let next_type = self.tokenizer.peek();

            if let Some(LineType::Quote) = next_type {
                let line = self.tokenizer.consume(next_type.unwrap()).unwrap();

                accumulator.add(&line.value().unwrap());
            } else {
                break;
            }
        }

        Some(Block::Quote { content: accumulator.consume() })
    }

    fn parse_heading(&mut self, line_type: LineType) -> Option<Block> {
        let level = match line_type {
            LineType::Heading1 => HeadingLevel::Level1,
            LineType::Heading2 => HeadingLevel::Level2,
            LineType::Heading3 => HeadingLevel::Level3,
            _ => return None,
        };

        let content = self.tokenizer.consume(line_type)?.value()?.trim().into();

        Some(Block::Heading { content, level })
    }
}

impl Iterator for BlockParser {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.peek() {
                None => return None,
                Some(LineType::Blank) => {
                    self.tokenizer.consume(LineType::Blank);
                    continue;
                }
                Some(LineType::Text) => return self.parse_text(),
                Some(LineType::Quote) => return self.parse_quote(),
                Some(LineType::Heading1) => return self.parse_heading(LineType::Heading1),
                Some(LineType::Heading2) => return self.parse_heading(LineType::Heading2),
                Some(LineType::Heading3) => return self.parse_heading(LineType::Heading3),
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

    #[test]
    fn it_works() {
        let mut parser = BlockParser::new("Lorem ipsum\ndolor sit amet");

        assert_eq!(
            Some(Block::Text {
                content: "Lorem ipsum dolor sit amet".to_string(),
            }),
            parser.next()
        );
    }

    #[test]
    fn parsing_headings_works() {
        let mut parser = BlockParser::new("# hello world\n##    level 2\n### three");

        assert_eq!(
            Some(Block::Heading {
                content: "hello world".into(),
                level: HeadingLevel::Level1,
            }),
            parser.next()
        );

        assert_eq!(
            Some(Block::Heading {
                content: "level 2".into(),
                level: HeadingLevel::Level2,
            }),
            parser.next()
        );
    }
}