use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
  Blank,
  Divider,
  Heading1(String),
  Heading2(String),
  Heading3(String),
  Text(String),
  Quote(String),
  Annotation(String),
  UnorderedList(String),
  OrderedList(String),
}

#[derive(Debug)]
pub struct BlockTokenizer {
  lines: VecDeque<String>,
}

impl BlockTokenizer {
  pub fn new<S: Into<String>>(input: S) -> Self {
    // TODO: define behaviour around \r\n
    let lines = input.into()
                     .split('\n')
                     .map(|line| line.to_string())
                     .collect();

    BlockTokenizer { lines }
  }

  pub fn consume_raw_line(&mut self) -> Option<String> {
    self.lines.pop_front()
  }

  pub fn consume_line(&mut self) -> Option<Line> {
    match self.consume_raw_line() {
      None => { return None }
      Some(line) => {
        // TODO: optimize
        let trimmed_line = line.trim().to_string();

        if trimmed_line.len() == 0 {
          return Some(Line::Blank);
        }

        if line.starts_with("# ") {
          return Some(Line::Heading1(
            line.chars()
                .skip(2)
                .collect()
          ))
        }

        if line.starts_with("## ") {
          return Some(Line::Heading2(
            line.chars()
                .skip(3)
                .collect()
          ))
        }

        if line.starts_with("### ") {
          return Some(Line::Heading3(
            line.chars()
                .skip(4)
                .collect()
          ))
        }

        if line.starts_with("> ") {
          return Some(Line::Quote(
            line.chars()
                .skip(2)
                .collect()
          ))
        }

        if line.starts_with("- ") {
          return Some(Line::UnorderedList(
            line.chars()
                .skip(2)
                .collect()
          ))
        }

        if line.starts_with(". ") {
          return Some(Line::OrderedList(
            line.chars()
                .skip(2)
                .collect()
          ))
        }

        if line.starts_with('[') && trimmed_line.ends_with(']') {
          return Some(Line::Annotation(
            trimmed_line.chars()
                        .skip(1)
                        .take(trimmed_line.len() - 2)
                        .collect()
          ));
        }

        if line.starts_with("---") && trimmed_line.chars().all(|c| c == '-') {
          return Some(Line::Divider);
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

        assert_eq!(tokenizer.consume_line(), Some(Line::Heading1("hello".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("world".into())));
    }

    #[test]
    fn heading_2_works() {
        let mut tokenizer = BlockTokenizer::new("## heading 2");

        assert_eq!(tokenizer.consume_line(), Some(Line::Heading2("heading 2".into())));
    }

    #[test]
    fn heading_3_works() {
        let mut tokenizer = BlockTokenizer::new("### lorem ipsum");

        assert_eq!(tokenizer.consume_line(), Some(Line::Heading3("lorem ipsum".into())));
    }

    #[test]
    fn text_with_hash_works() {
        let mut tokenizer = BlockTokenizer::new(" # lorem ipsum");

        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" # lorem ipsum".into())));
    }

    #[test]
    fn quote_works() {
        let mut tokenizer = BlockTokenizer::new("> quote\n > quote\n>quote");

        assert_eq!(tokenizer.consume_line(), Some(Line::Quote("quote".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" > quote".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(">quote".into())));
    }

    #[test]
    fn annotation_works() {
        let mut tokenizer = BlockTokenizer::new("[code]\n[code]   \n [code] \n  [code]  \n[code");

        assert_eq!(tokenizer.consume_line(), Some(Line::Annotation("code".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Annotation("code".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" [code] ".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("  [code]  ".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("[code".into())));
    }

    #[test]
    fn unordered_list_works() {
        let mut tokenizer = BlockTokenizer::new("- item\n - item\n-item");

        assert_eq!(tokenizer.consume_line(), Some(Line::UnorderedList("item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text(" - item".into())));
        assert_eq!(tokenizer.consume_line(), Some(Line::Text("-item".into())));
    }

    #[test]
    fn ordered_list_works() {
        let mut tokenizer = BlockTokenizer::new(".  item\n . item\n.item");

        assert_eq!(tokenizer.consume_line(), Some(Line::OrderedList(" item".into())));
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
      assert_eq!(tokenizer.consume_line(), Some(Line::Text("---foobar".into())));
    }
}
