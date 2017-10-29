use super::constants;
use super::tokens::{Line, LineType};
use super::input::{ParserInputResult, IntoParserInput, IntoParserInputIter};
use super::error::ParseError;
use std::iter;
use std::str::Lines;
use std::borrow::Cow;

macro_rules! parse_starter {
  ($line:expr, $starter:expr, $variant: ident) => {
    if $line.starts_with($starter) {
        Some(Ok(Line::$variant(
            Cow::Owned($line.chars().skip($starter.len()).collect())
        )))
    } else {
        None
    }
  }
}

macro_rules! detect_line_starter {
  ($line:expr, $starter:expr, $variant: ident) => {
    if $line.starts_with($starter) {
      return LineType::$variant;
    }
  }
}

#[derive(Debug)]
pub struct BlockTokenizer<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    input: iter::Peekable<IntoParserInputIter<'a, S, I>>,
}

fn parse_decorator<'a>(line: Cow<'a, str>) -> Line {
    let trimmed = line.trim();

    Line::Decorator(trimmed.chars().skip(1).take(trimmed.len() - 2).collect())
}

fn is_decorator(line: &str) -> bool {
    line.starts_with(constants::ANNOTATION_PREFIX_TOKEN) &&
        line.trim().ends_with(constants::ANNOTATION_SUFFIX_TOKEN)
}

fn is_divider(line: &str) -> bool {
    line.starts_with("---") && line.trim().chars().all(|c| c == '-')
}

pub fn is_blank(line: &str) -> bool {
    line.chars().all(char::is_whitespace)
}

fn get_line_type(line: &str) -> LineType {
    if is_divider(line) {
        return LineType::Divider;
    }

    if is_decorator(line) {
        return LineType::Decorator;
    }

    if is_blank(line) {
        return LineType::Blank;
    }

    detect_line_starter!(line, constants::HEADING1_TOKEN, Heading1);
    detect_line_starter!(line, constants::HEADING2_TOKEN, Heading2);
    detect_line_starter!(line, constants::HEADING3_TOKEN, Heading3);
    detect_line_starter!(line, constants::QUOTE_TOKEN, Quote);
    detect_line_starter!(line, constants::UNORDERED_LIST_TOKEN, UnorderedList);
    detect_line_starter!(line, constants::ORDERED_LIST_TOKEN, OrderedList);

    LineType::Text
}

impl<'a> BlockTokenizer<'a, &'a str, Lines<'a>> {
    pub fn from_string(input: &'a str) -> Self {
        BlockTokenizer::new(input.lines())
    }
}

impl<'a, S, I> BlockTokenizer<'a, S, I>
where
    S: IntoParserInput<'a>,
    I: Iterator<Item = S>,
{
    pub fn new(input: I) -> Self {
        BlockTokenizer { input: IntoParserInputIter::new(input).peekable() }
    }

    pub fn peek(&mut self) -> Option<Result<LineType, ParseError>> {
        let result = match self.input.peek()? {
            &Err(_) => Err(ParseError::PeekError),
            &Ok(ref line) => Ok(get_line_type(line)),
        };

        Some(result)
    }

    pub fn consume(&mut self, line_type: LineType) -> Option<Result<Line<'a>, ParseError>> {
        match self.consume_raw()? {
            Err(err) => Some(Err(err)),
            Ok(line) => {
                match line_type {
                    LineType::Blank => Some(Ok(Line::Blank)),
                    LineType::Divider => Some(Ok(Line::Divider)),
                    LineType::Text => Some(Ok(Line::Text(line))),
                    LineType::Decorator => Some(Ok(parse_decorator(line))),
                    LineType::Heading1 => parse_starter!(line, constants::HEADING1_TOKEN, Heading1),
                    LineType::Heading2 => parse_starter!(line, constants::HEADING2_TOKEN, Heading2),
                    LineType::Heading3 => parse_starter!(line, constants::HEADING3_TOKEN, Heading3),
                    LineType::Quote => parse_starter!(line, constants::QUOTE_TOKEN, Quote),
                    LineType::UnorderedList => {
                        parse_starter!(line, constants::UNORDERED_LIST_TOKEN, UnorderedList)
                    }
                    LineType::OrderedList => {
                        parse_starter!(line, constants::ORDERED_LIST_TOKEN, OrderedList)
                    }
                }
            }
        }
    }

    pub fn consume_raw(&mut self) -> Option<ParserInputResult<'a>> {
        self.input.next()
    }

    pub fn consume_line(&mut self) -> Option<Result<Line<'a>, ParseError>> {
        let result = match self.peek()? {
            Err(err) => Err(err),
            Ok(line_type) => self.consume(line_type)?,
        };

        Some(result)
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
    fn text_works() {
        let mut tokenizer = BlockTokenizer::from_string("hello\nworld");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("hello".into())
        );

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("world".into())
        );
    }

    #[test]
    fn heading_1_works() {
        let mut tokenizer = BlockTokenizer::from_string("# hello\nworld");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Heading1("hello".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("world".into())
        );
    }

    #[test]
    fn heading_2_works() {
        let mut tokenizer = BlockTokenizer::from_string("## heading 2");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Heading2("heading 2".into())
        );
    }

    #[test]
    fn heading_3_works() {
        let mut tokenizer = BlockTokenizer::from_string("### lorem ipsum");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Heading3("lorem ipsum".into())
        );
    }

    #[test]
    fn text_with_hash_works() {
        let mut tokenizer = BlockTokenizer::from_string(" # lorem ipsum");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(" # lorem ipsum".into())
        );
    }

    #[test]
    fn quote_works() {
        let mut tokenizer = BlockTokenizer::from_string("> quote\n > quote\n>quote");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Quote("quote".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(" > quote".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(">quote".into())
        );
    }

    #[test]
    fn decorator_works() {
        let mut tokenizer =
            BlockTokenizer::from_string("[code]\n[code]   \n [code] \n  [code]  \n[code");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Decorator("code".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Decorator("code".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(" [code] ".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("  [code]  ".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("[code".into())
        );
    }

    #[test]
    fn unordered_list_works() {
        let mut tokenizer = BlockTokenizer::from_string("- item\n - item\n-item");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::UnorderedList("item".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(" - item".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("-item".into())
        );
    }

    #[test]
    fn ordered_list_works() {
        let mut tokenizer = BlockTokenizer::from_string(".  item\n . item\n.item");

        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::OrderedList(" item".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(" . item".into())
        );
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text(".item".into())
        );
    }

    #[test]
    fn divider_works() {
        let mut tokenizer = BlockTokenizer::from_string("---\n------- \n--\n ---\n---foobar");

        assert_eq!(unwrap!(tokenizer.consume_line()), Line::Divider);
        assert_eq!(unwrap!(tokenizer.consume_line()), Line::Divider);
        assert_eq!(unwrap!(tokenizer.consume_line()), Line::Text("--".into()));
        assert_eq!(unwrap!(tokenizer.consume_line()), Line::Text(" ---".into()));
        assert_eq!(
            unwrap!(tokenizer.consume_line()),
            Line::Text("---foobar".into())
        );
    }

    #[test]
    fn empty_works() {
        let mut tokenizer = BlockTokenizer::from_string("   \t");

        assert_eq!(unwrap!(tokenizer.consume_line()), Line::Blank);
    }
}
