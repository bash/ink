use std::iter::Peekable;
use std::str::Lines;
use std::error::Error;
use std::marker;
use super::constants;
use super::err::ParseError;

pub type IterItem<'a> = Result<&'a str, ParseError>;

pub trait IntoIterItem<'a> {
    fn into_iter_item(self) -> IterItem<'a>;
    fn into_iter_item_ref(&self) -> IterItem<'a>;
}

impl<'a, E> IntoIterItem<'a> for Result<&'a str, E>
where
    E: Error + Clone + 'static,
{
    fn into_iter_item(self) -> IterItem<'a> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(ParseError::InputError(Box::new(err))),
        }
    }

    fn into_iter_item_ref(&self) -> IterItem<'a> {
        match self {
            &Ok(value) => Ok(value),
            &Err(ref err) => Err(ParseError::InputError(Box::new(err.clone()))),
        }
    }
}

impl<'a> IntoIterItem<'a> for &'a str {
    fn into_iter_item(self) -> IterItem<'a> {
        Ok(self)
    }

    fn into_iter_item_ref(&self) -> IterItem<'a> {
        Ok(self)
    }
}

macro_rules! parse_starter {
  ($line:expr, $starter:expr, $variant: ident) => {
    if $line.starts_with($starter) {
        Some(Ok(Line::$variant(
            $line.chars()
                .skip($starter.len())
                .collect()
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

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Blank,
    Divider,
    Heading1(String),
    Heading2(String),
    Heading3(String),
    Text(String),
    Quote(String),
    Decorator(String),
    UnorderedList(String),
    OrderedList(String),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LineType {
    Blank,
    Divider,
    Heading1,
    Heading2,
    Heading3,
    Text,
    Quote,
    Decorator,
    UnorderedList,
    OrderedList,
}

#[derive(Debug)]
pub struct BlockTokenizer<'a, A, I>
where
    A: IntoIterItem<'a>,
    I: Iterator<Item = A>,
{
    inner: Peekable<I>,
    _marker: marker::PhantomData<&'a ()>,
}

fn parse_decorator(line: &str) -> Line {
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

impl Line {
    pub fn value(self) -> Option<String> {
        match self {
            Line::Blank => None,
            Line::Divider => None,
            Line::Heading1(value) => Some(value),
            Line::Heading2(value) => Some(value),
            Line::Heading3(value) => Some(value),
            Line::Text(value) => Some(value),
            Line::Quote(value) => Some(value),
            Line::Decorator(value) => Some(value),
            Line::UnorderedList(value) => Some(value),
            Line::OrderedList(value) => Some(value),
        }
    }
}

impl<'a, A, I> BlockTokenizer<'a, A, I>
where
    A: IntoIterItem<'a>,
    I: Iterator<Item = A>,
{
    pub fn new(input: I) -> Self {
        let inner = input.peekable();

        BlockTokenizer {
            inner,
            _marker: marker::PhantomData,
        }
    }

    pub fn from_string<'b>(input: &'b str) -> BlockTokenizer<'b, &'b str, Lines<'b>> {
        BlockTokenizer::new(input.lines())
    }

    pub fn peek(&'a mut self) -> Option<Result<LineType, ParseError>> {
        Some(self.inner.peek()?.into_iter_item_ref().map(|line| {
            get_line_type(line)
        }))
    }

    pub fn consume(&'a mut self, line_type: LineType) -> Option<Result<Line, ParseError>> {
        match self.consume_raw()? {
            Err(err) => Some(Err(err)),
            Ok(line) => {
                match line_type {
                    LineType::Blank => Some(Ok(Line::Blank)),
                    LineType::Divider => Some(Ok(Line::Divider)),
                    LineType::Text => Some(Ok(Line::Text(line.to_string()))),
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

    pub fn consume_raw(&'a mut self) -> Option<IterItem<'a>> {
        Some(self.inner.next()?.into_iter_item())
    }

    pub fn consume_line(&'a mut self) -> Option<Result<Line, ParseError>> {
        let line_type = self.peek()?;

        let result = match line_type {
            Ok(line_type) => self.consume(line_type)?,
            Err(err) => Err(err),
        };

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_works() {
        let mut tokenizer = BlockTokenizer::from_string("hello\nworld");

        assert_eq!(
            tokenizer.consume_line().unwrap(),
            Some(Line::Text("hello".into()))
        );
        assert_eq!(
            tokenizer.consume_line().unwrap(),
            Some(Line::Text("world".into()))
        );
    }

    #[test]
    fn heading_1_works() {
        let mut tokenizer = BlockTokenizer::from_string("# hello\nworld");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading1("hello".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("world".into())));
    }

    #[test]
    fn heading_2_works() {
        let mut tokenizer = BlockTokenizer::from_string("## heading 2");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading2("heading 2".into()))
        );
    }

    #[test]
    fn heading_3_works() {
        let mut tokenizer = BlockTokenizer::from_string("### lorem ipsum");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading3("lorem ipsum".into()))
        );
    }

    #[test]
    fn text_with_hash_works() {
        let mut tokenizer = BlockTokenizer::from_string(" # lorem ipsum");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text(" # lorem ipsum".into()))
        );
    }

    #[test]
    fn quote_works() {
        let mut tokenizer = BlockTokenizer::from_string("> quote\n > quote\n>quote");

        assert_eq!(tokenizer.consume_line(), Some(Line::Quote("quote".into())));
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text(" > quote".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(">quote".into())));
    }

    #[test]
    fn decorator_works() {
        let mut tokenizer =
            BlockTokenizer::from_string("[code]\n[code]   \n [code] \n  [code]  \n[code");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Decorator("code".into()))
        );
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Decorator("code".into()))
        );
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text(" [code] ".into()))
        );
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text("  [code]  ".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("[code".into())));
    }

    #[test]
    fn unordered_list_works() {
        let mut tokenizer = BlockTokenizer::from_string("- item\n - item\n-item");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::UnorderedList("item".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" - item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("-item".into())));
    }

    #[test]
    fn ordered_list_works() {
        let mut tokenizer = BlockTokenizer::from_string(".  item\n . item\n.item");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::OrderedList(" item".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" . item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(".item".into())));
    }

    #[test]
    fn divider_works() {
        let mut tokenizer = BlockTokenizer::from_string("---\n------- \n--\n ---\n---foobar");

        assert_eq!(tokenizer.consume_line(), Some(Line::Divider));
        assert_eq!(tokenizer.consume_line(), Some(Line::Divider));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("--".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" ---".into())));
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text("---foobar".into()))
        );
    }

    #[test]
    fn empty_works() {
        let mut tokenizer = BlockTokenizer::from_string("   \t");

        assert_eq!(tokenizer.consume_line(), Some(Line::Blank));
    }
}
