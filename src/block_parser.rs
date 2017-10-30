use super::block_tokenizer::BlockTokenizer;
use super::tokens::LineType;
use super::ast::{Block, HeadingLevel, Inline, Text};
use super::input::IntoParserInput;
use super::error::ParseError;
use std::str::Lines;

macro_rules! consume_error {
    ($tokenizer:expr) => {
        match $tokenizer.consume_raw()? {
            Err(err) => return Some(Err(err)),
            // If peek() returns an error, we know that
            // consume_raw() must return an error too.
            Ok(..) => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct BlockParser<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    tokenizer: BlockTokenizer<'a, S, I>,
}

#[derive(Debug)]
pub struct TextAccumulator {
    buffer: String,
}

impl<'a> BlockParser<'a, &'a str, Lines<'a>> {
    pub fn from_string(input: &'a str) -> Self {
        BlockParser { tokenizer: BlockTokenizer::from_string(input) }
    }
}

impl<'a, S, I> BlockParser<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    pub fn new(input: I) -> Self {
        BlockParser { tokenizer: BlockTokenizer::new(input) }
    }

    fn parse_text(&mut self) -> Option<Result<Block, ParseError>> {
        let mut accumulator = TextAccumulator::new();

        loop {
            match self.tokenizer.peek() {
                None => break,
                Some(Err(..)) => consume_error!(self.tokenizer),
                Some(Ok(LineType::Text)) => {
                    // unwrapping here is safe
                    let line = self.tokenizer.consume(LineType::Text).unwrap().unwrap();

                    accumulator.add(&line.value().unwrap());
                }
                Some(Ok(_)) => break,
            }
        }

        Some(Ok(Block::Paragraph(accumulator.consume())))
    }

    fn parse_quote(&mut self) -> Option<Result<Block, ParseError>> {
        let mut accumulator = TextAccumulator::new();

        loop {
            match self.tokenizer.peek() {
                None => break,
                Some(Err(..)) => consume_error!(self.tokenizer),
                Some(Ok(LineType::Quote)) => {
                    // unwrapping here is safe
                    let line = self.tokenizer.consume(LineType::Quote).unwrap().unwrap();

                    accumulator.add(&line.value().unwrap());
                }
                Some(Ok(_)) => break,
            }
        }

        Some(Ok(Block::Quote(accumulator.consume())))
    }

    fn parse_heading(&mut self, line_type: LineType) -> Option<Result<Block, ParseError>> {
        let level = match line_type {
            LineType::Heading1 => HeadingLevel::Level1,
            LineType::Heading2 => HeadingLevel::Level2,
            LineType::Heading3 => HeadingLevel::Level3,
            _ => unreachable!(),
        };

        match self.tokenizer.consume(line_type)? {
            Err(err) => Some(Err(err)),
            Ok(line) => Some(Ok(Block::Heading(level, line.value()?.trim().into()))),
        }
    }
}

impl<'a, S, I> Iterator for BlockParser<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    type Item = Result<Block, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.peek()? {
                Err(..) => consume_error!(self.tokenizer),
                Ok(LineType::Blank) => {
                    self.tokenizer.consume_raw();
                    continue;
                }
                Ok(line_type) => {
                    return match line_type {
                        LineType::Text => self.parse_text(),
                        LineType::Quote => self.parse_quote(),
                        LineType::Heading1 => self.parse_heading(LineType::Heading1),
                        LineType::Heading2 => self.parse_heading(LineType::Heading2),
                        LineType::Heading3 => self.parse_heading(LineType::Heading3),
                        _ => unimplemented!(),
                    };
                }
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
    ///
    pub fn add(&mut self, line: &str) {
        if self.buffer.len() > 0 {
            self.buffer.push_str(" ");
        }

        self.buffer.push_str(line.trim());
    }

    pub fn consume(self) -> Text {
        vec![Inline::Chunk(self.buffer)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! unwrap {
        ($value:expr) => {
            $value.unwrap().unwrap()
        }
    }

    #[test]
    fn it_works() {
        let mut parser = BlockParser::from_string("Lorem ipsum\ndolor sit amet");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("Lorem ipsum dolor sit amet".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn parsing_headings_works() {
        let mut parser = BlockParser::from_string("# hello world\n##    level 2\n### three");

        assert_eq!(
            Block::Heading(HeadingLevel::Level1, "hello world".into()),
            unwrap!(parser.next())
        );

        assert_eq!(
            Block::Heading(HeadingLevel::Level2, "level 2".into()),
            unwrap!(parser.next())
        );

        assert_eq!(
            Block::Heading(HeadingLevel::Level3, "three".into()),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn parsing_quote_works() {
        let mut parser = BlockParser::from_string("> Foo\n> bar baz");

        assert_eq!(
            Block::Quote(vec![Inline::Chunk("Foo bar baz".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn parsing_text_works() {
        let mut parser = BlockParser::from_string("Foo\n    \tbar baz");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("Foo bar baz".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn blank_lines_are_ignored() {
        let mut parser = BlockParser::from_string("   \n \t \nfoo");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("foo".into())]),
            unwrap!(parser.next())
        );
    }
}