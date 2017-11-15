use super::constants;
use super::tokens::{Line, LineType};
use super::ast::Decorator;
use std::borrow::Cow;

macro_rules! parse_line_starter {
    ($line: expr, $starter: expr, $variant: ident) => {
        Line::$variant(
            Cow::Owned($line.chars().skip($starter.len()).collect())
        )
    }
}

macro_rules! parse_line_starter2 {
    ($line: expr, $starter: expr, $variant: ident) => {
        $line.chars().skip($starter.len()).collect()
    }
}


macro_rules! detect_line_starter {
  ($line:expr, $starter:expr, $variant: ident) => {
    if $line.starts_with($starter) {
      return LineType::$variant;
    }
  }
}

// TODO: parse decorator contents
pub fn parse_decorator(line: &str) -> Option<Decorator> {
    let trimmed = line.trim();

    let value: String = trimmed.chars().skip(1).take(trimmed.len() - 2).collect();

    match value.as_ref() {
        // TODO: parse language
        "code" => Some(Decorator::Code(None)),
        "table" => Some(Decorator::Table),
        _ => None,
    }
}

fn is_decorator(line: &str) -> bool {
    line.starts_with(constants::ANNOTATION_PREFIX_TOKEN)
        && line.trim().ends_with(constants::ANNOTATION_SUFFIX_TOKEN)
}

fn is_divider(line: &str) -> bool {
    line.starts_with("---") && line.trim().chars().all(|c| c == '-')
}

fn is_blank(line: &str) -> bool {
    line.chars().all(char::is_whitespace)
}

pub fn get_line_type(line: &str) -> LineType {
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

pub fn parse_quote(line: &str) -> String {
    parse_line_starter2!(line, constants::QUOTE_TOKEN, Quote)
}

#[deprecated]
pub fn parse_line(line_type: LineType, line: &str) -> Line {
    match line_type {
        LineType::Blank => Line::Blank,
        LineType::Divider => Line::Divider,
        LineType::Text => Line::Text(line.into()),
        LineType::Decorator => unreachable!(),
        LineType::Heading1 => parse_line_starter!(line, constants::HEADING1_TOKEN, Heading1),
        LineType::Heading2 => parse_line_starter!(line, constants::HEADING2_TOKEN, Heading2),
        LineType::Heading3 => parse_line_starter!(line, constants::HEADING3_TOKEN, Heading3),
        LineType::Quote => parse_line_starter!(line, constants::QUOTE_TOKEN, Quote),
        LineType::UnorderedList => {
            parse_line_starter!(line, constants::UNORDERED_LIST_TOKEN, UnorderedList)
        }
        LineType::OrderedList => {
            parse_line_starter!(line, constants::ORDERED_LIST_TOKEN, OrderedList)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_line {
        ($expected: expr, $line: expr) => {
            assert_eq!(
                parse_line(get_line_type($line), $line),
                $expected
            );
        }
    }

    #[test]
    fn text_works() {
        test_parse_line!(Line::Text("hello world".into()), "hello world");
    }

    #[test]
    fn heading_1_works() {
        test_parse_line!(Line::Heading1("hello world".into()), "# hello world");
        test_parse_line!(Line::Text("#hello world".into()), "#hello world");
    }

    #[test]
    fn heading_2_works() {
        test_parse_line!(Line::Heading2("hello world".into()), "## hello world");
        test_parse_line!(Line::Text("##hello world".into()), "##hello world");
    }

    #[test]
    fn heading_3_works() {
        test_parse_line!(Line::Heading3("hello world".into()), "### hello world");
        test_parse_line!(Line::Text("###hello world".into()), "###hello world");
    }

    #[test]
    fn text_with_hash_works() {
        test_parse_line!(Line::Text(" ## hello world".into()), " ## hello world");
    }

    #[test]
    fn quote_works() {
        test_parse_line!(Line::Quote("quote".into()), "> quote");
        test_parse_line!(Line::Text(">quote".into()), ">quote");
        test_parse_line!(Line::Text(" > quote".into()), " > quote");
    }

    #[test]
    fn decorator_works() {
        test_parse_line!(Line::Decorator("code".into()), "[code]");
        test_parse_line!(Line::Decorator("code".into()), "[code]      ");
        test_parse_line!(Line::Text(" [code]      ".into()), " [code]      ");
        test_parse_line!(Line::Text("[code".into()), "[code");
    }

    #[test]
    fn unordered_list_works() {
        test_parse_line!(Line::UnorderedList("item".into()), "- item");
        test_parse_line!(Line::Text(" - item".into()), " - item");
        test_parse_line!(Line::Text("-item".into()), "-item");
    }

    #[test]
    fn ordered_list_works() {
        test_parse_line!(Line::OrderedList("item".into()), ". item");
        test_parse_line!(Line::Text(" . item".into()), " . item");
        test_parse_line!(Line::Text(".item".into()), ".item");
    }

    #[test]
    fn divider_works() {
        test_parse_line!(Line::Divider, "---");
        test_parse_line!(Line::Divider, "-------");
        test_parse_line!(Line::Text("--".into()), "--");
        test_parse_line!(Line::Text(" ---".into()), " ---");
        test_parse_line!(Line::Text("---foobar".into()), "---foobar");
    }

    #[test]
    fn blank_works() {
        test_parse_line!(Line::Blank, "");
        test_parse_line!(Line::Blank, "     ");
        test_parse_line!(Line::Blank, "\t");
        test_parse_line!(Line::Blank, "    \t   ");
    }
}
