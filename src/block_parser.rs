use super::block_tokenizer::{BlockTokenizer, LineType};
use std::str::Lines;
use std::error::Error;

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
    Quote { content: String },
    FencedBlock {
        decorator: Option<String>,
        content: String,
    },
    List {
        list_type: ListType,
        items: Vec<String>,
    },
}
/*
#[derive(Debug)]
pub struct BlockParser<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    tokenizer: BlockTokenizer<'a, I>,
}

#[derive(Debug)]
pub struct TextAccumulator {
    buffer: String,
}

impl<'a, I> BlockParser<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    pub fn new<S: IntoIterator<Item = &'a str, IntoIter = I>>(input: S) -> Self {
        BlockParser { tokenizer: BlockTokenizer::new(input) }
    }

    pub fn from_string<'b>(input: &'b str) -> BlockParser<'b, Lines<'b>> {
        BlockParser { tokenizer: BlockTokenizer::<'b, Lines<'b>>::from_string(input) }
    }

    fn parse_text(&mut self) -> Option<Block> {
        let mut accumulator = TextAccumulator::new();

        loop {
            match self.tokenizer.peek() {
                Some(LineType::Text) => {
                    accumulator.add(self.tokenizer.consume(LineType::Text)?.value()?);
                }
                _ => break,
            }
        }

        Some(Block::Text { content: accumulator.consume() })
    }

    fn parse_quote(&mut self) -> Option<Block> {
        let mut accumulator = TextAccumulator::new();

        loop {
            match self.tokenizer.peek() {
                Some(LineType::Quote) => {
                    accumulator.add(self.tokenizer.consume(LineType::Quote)?.value()?);
                }
                _ => break,
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

    fn parse_list(&mut self, line_type: LineType) -> Option<Block> {
        let mut items: Vec<String> = vec![];

        let list_type = match line_type {
            LineType::OrderedList => ListType::Ordered,
            LineType::UnorderedList => ListType::Unordered,
            _ => return None,
        };

        loop {
            match self.tokenizer.peek() {
                Some(next_type) => {
                    if next_type != line_type {
                        break;
                    }

                    items.push(self.tokenizer.consume(line_type)?.value()?);
                }
                None => break,
            }
        }

        Some(Block::List { list_type, items })
    }
}

impl<'a, I> Iterator for BlockParser<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.peek()? {
                // blank lines are skipped until a non-blank line is found.
                LineType::Blank => self.tokenizer.consume_raw(),
                LineType::Text => return self.parse_text(),
                LineType::Quote => return self.parse_quote(),
                LineType::Heading1 => return self.parse_heading(LineType::Heading1),
                LineType::Heading2 => return self.parse_heading(LineType::Heading2),
                LineType::Heading3 => return self.parse_heading(LineType::Heading3),
                LineType::OrderedList => return self.parse_list(LineType::OrderedList),
                LineType::UnorderedList => return self.parse_list(LineType::UnorderedList),
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
    pub fn add(&mut self, line: String) {
        if self.buffer.len() > 0 {
            self.buffer.push_str(" ");
        }

        self.buffer.push_str(&line);
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
        let mut parser = BlockParser::from_string("Lorem ipsum\ndolor sit amet");

        assert_eq!(
            Some(Block::Text {
                content: "Lorem ipsum dolor sit amet".to_string(),
            }),
            parser.next()
        );
    }

    #[test]
    fn parsing_headings_works() {
        let parser = BlockParser::from_string("# hello world\n##    level 2\n### three");

        assert_eq!(
            vec![
                Block::Heading {
                    content: "hello world".into(),
                    level: HeadingLevel::Level1,
                },
                Block::Heading {
                    content: "level 2".into(),
                    level: HeadingLevel::Level2,
                },
                Block::Heading {
                    content: "three".into(),
                    level: HeadingLevel::Level3,
                },
            ],
            parser.collect::<Vec<Block>>()
        );
    }

    #[test]
    fn parsing_lists_work() {
        let parser = BlockParser::from_string(
            r#"
. uno
. due
. tres
- apples
- oranges
- pears
"#,
        );

        assert_eq!(
            vec![
                Block::List {
                    list_type: ListType::Ordered,
                    items: vec!["uno".into(), "due".into(), "tres".into()],
                },
                Block::List {
                    list_type: ListType::Unordered,
                    items: vec!["apples".into(), "oranges".into(), "pears".into()],
                },
            ],
            parser.collect::<Vec<Block>>()
        );
    }
}
*/