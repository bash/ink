use std::collections::VecDeque;

macro_rules! parse_line_starter {
  ($line:expr, $starter:expr, $variant: ident) => {
    if $line.starts_with($starter) {
      return Some(Line::$variant(
        $line.chars()
             .skip($starter.len())
             .collect()
      ))
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

#[derive(Debug, PartialEq, Eq)]
pub enum RawLine {
    Divider,
    Text(String),
}

#[derive(Debug)]
pub struct BlockTokenizer {
    lines: VecDeque<String>,
}

fn parse_annotation(line: &str) -> Option<Line> {
    use super::constants;

    let trimmed = line.trim();

    if line.starts_with(constants::ANNOTATION_PREFIX_TOKEN) &&
        trimmed.ends_with(constants::ANNOTATION_SUFFIX_TOKEN)
    {
        return Some(Line::Decorator(
            trimmed.chars().skip(1).take(trimmed.len() - 2).collect(),
        ));
    }

    None
}

impl BlockTokenizer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let lines = input.into().lines().map(|line| line.to_string()).collect();

        BlockTokenizer { lines }
    }

    pub fn consume_raw_line(&mut self) -> Option<RawLine> {
        match self.lines.pop_front() {
            None => None,
            Some(line) => {
                if line.starts_with("---") && line.trim().chars().all(|c| c == '-') {
                    return Some(RawLine::Divider);
                }

                Some(RawLine::Text(line))
            }
        }
    }

    pub fn consume_line(&mut self) -> Option<Line> {
        match self.consume_raw_line() {
            None => None,
            Some(RawLine::Divider) => Some(Line::Divider),
            Some(RawLine::Text(line)) => {
                if line.chars().all(char::is_whitespace) {
                    return Some(Line::Blank);
                }

                use super::constants;

                parse_line_starter!(line, constants::HEADING1_TOKEN, Heading1);
                parse_line_starter!(line, constants::HEADING2_TOKEN, Heading2);
                parse_line_starter!(line, constants::HEADING3_TOKEN, Heading3);
                parse_line_starter!(line, constants::QUOTE_TOKEN, Quote);
                parse_line_starter!(line, constants::UNORDERED_LIST_TOKEN, UnorderedList);
                parse_line_starter!(line, constants::ORDERED_LIST_TOKEN, OrderedList);

                if let Some(line) = parse_annotation(&line) {
                    return Some(line);
                }

                Some(Line::Text(line))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_works() {
        let mut tokenizer = BlockTokenizer::new("hello\nworld");

        assert_eq!(tokenizer.consume_line(), Some(Line::Text("hello".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("world".into())));
    }

    #[test]
    fn heading_1_works() {
        let mut tokenizer = BlockTokenizer::new("# hello\nworld");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading1("hello".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("world".into())));
    }

    #[test]
    fn heading_2_works() {
        let mut tokenizer = BlockTokenizer::new("## heading 2");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading2("heading 2".into()))
        );
    }

    #[test]
    fn heading_3_works() {
        let mut tokenizer = BlockTokenizer::new("### lorem ipsum");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Heading3("lorem ipsum".into()))
        );
    }

    #[test]
    fn text_with_hash_works() {
        let mut tokenizer = BlockTokenizer::new(" # lorem ipsum");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text(" # lorem ipsum".into()))
        );
    }

    #[test]
    fn quote_works() {
        let mut tokenizer = BlockTokenizer::new("> quote\n > quote\n>quote");

        assert_eq!(tokenizer.consume_line(), Some(Line::Quote("quote".into())));
        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::Text(" > quote".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(">quote".into())));
    }

    #[test]
    fn decorator_works() {
        let mut tokenizer = BlockTokenizer::new("[code]\n[code]   \n [code] \n  [code]  \n[code");

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
        let mut tokenizer = BlockTokenizer::new("- item\n - item\n-item");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::UnorderedList("item".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" - item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("-item".into())));
    }

    #[test]
    fn ordered_list_works() {
        let mut tokenizer = BlockTokenizer::new(".  item\n . item\n.item");

        assert_eq!(
            tokenizer.consume_line(),
            Some(Line::OrderedList(" item".into()))
        );
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" . item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(".item".into())));
    }

    #[test]
    fn divider_works() {
        let mut tokenizer = BlockTokenizer::new("---\n------- \n--\n ---\n---foobar");

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
        let mut tokenizer = BlockTokenizer::new("   \t");

        assert_eq!(tokenizer.consume_line(), Some(Line::Blank));
    }
}
