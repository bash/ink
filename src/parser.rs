use super::block_tokenizer::{get_line_type, parse_line};
use super::tokens::LineType;
use super::ast::Block;
use super::input::{IntoParserInput, IntoParserInputIter};
use super::error::ParseError;
use super::block_parser::BlockParser;
use std::str::Lines;
use std::iter::Peekable;

macro_rules! opt_result_try {
    ($result: expr) => {
        match $result? {
            Err(err) => return Some(Err(err.into())),
            Ok(val) => val,
        }
    }
}

#[derive(Debug)]
pub struct Parser<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    input: Peekable<IntoParserInputIter<S, I>>,
}

impl<'a> Parser<&'a str, Lines<'a>> {
    pub fn from_string(input: &'a str) -> Self {
        Parser {
            input: IntoParserInputIter::new(input.lines()).peekable(),
        }
    }
}

impl<S, I> Parser<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    pub fn new(input: I) -> Self {
        Parser {
            input: IntoParserInputIter::new(input).peekable(),
        }
    }

    fn peek(&mut self) -> Option<Result<LineType, ()>> {
        match *self.input.peek()? {
            Ok(ref line) => Some(Ok(get_line_type(line))),
            Err(_) => Some(Err(())),
        }
    }

    fn parse_heading(&self, line_type: LineType, line: String) -> Block {
        let level = line_type
            .get_heading_level()
            .expect("heading level should be defined for line type");

        let line = parse_line(line_type, &line);

        Block::Heading(level, line.value().unwrap().trim().into())
    }
}

impl<S, I> Iterator for Parser<S, I>
where
    S: IntoParserInput,
    I: Iterator<Item = S>,
{
    type Item = Result<Block, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = opt_result_try!(self.input.next());
            let line_type = get_line_type(&line);

            // empty lines never produce anything so we skip them
            if let LineType::Blank = line_type {
                continue;
            }

            // headings are always only a single line, so it doesn't
            // make sense to create a processor for it.
            if line_type.is_heading() {
                return Some(Ok(self.parse_heading(line_type, line)));
            }

            let mut parser = BlockParser::for_line_type(line_type);

            parser.processor_mut().process_line(line_type, line);

            loop {
                let line_type = match self.peek() {
                    None => break,
                    Some(Err(_)) => return Some(Err(self.input.next().unwrap().unwrap_err())),
                    Some(Ok(line_type)) => line_type,
                };

                if !parser.processor().can_process(line_type) {
                    break;
                }

                let line = opt_result_try!(self.input.next());

                parser.processor_mut().process_line(line_type, line.into())
            }

            return Some(Ok(parser.consume()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ast::{HeadingLevel, Inline};

    macro_rules! unwrap {
        ($value:expr) => {
            $value.unwrap().unwrap()
        }
    }

    #[test]
    fn it_works() {
        let mut parser = Parser::from_string("Lorem ipsum\ndolor sit amet");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("Lorem ipsum dolor sit amet".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn parsing_headings_works() {
        let mut parser = Parser::from_string("# hello world\n##    level 2\n### three");

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
        let mut parser = Parser::from_string("> Foo\n> bar baz");

        assert_eq!(
            Block::Quote(vec![Inline::Chunk("Foo bar baz".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn parsing_text_works() {
        let mut parser = Parser::from_string("Foo\n    \tbar baz");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("Foo bar baz".into())]),
            unwrap!(parser.next())
        );
    }

    #[test]
    fn blank_lines_are_ignored() {
        let mut parser = Parser::from_string("   \n \t \nfoo");

        assert_eq!(
            Block::Paragraph(vec![Inline::Chunk("foo".into())]),
            unwrap!(parser.next())
        );
    }
}
