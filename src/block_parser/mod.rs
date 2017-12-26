use super::tokens::LineType;
use super::ast::{Block, Inline, Text};
use std::fmt;

#[macro_export]
macro_rules! accept {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => InspectOutput::Accept,
            _ => InspectOutput::Reject,
        }
    }
}

mod paragraph;
mod quote;
mod pre;

use self::paragraph::ParagraphParser;
use self::quote::QuoteParser;
use self::pre::PreParser;

#[derive(Debug)]
pub struct TextAccumulator {
    buffer: String,
}

#[derive(Debug, Copy, Clone)]
pub enum InspectOutput {
    /// Accepts the given line
    Accept,
    /// Accepts this line, but will reject all next lines
    Last,
    /// Rejects this line
    Reject,
    /// Forces the consumer to fall back to plain text processing for the whole block
    Fallback,
}

#[derive(Debug, Copy, Clone)]
pub enum TakeOutput {
    /// Accepts the given line
    Accepted,
    /// Accepts this line, but will reject all next lines
    Last,
    /// Forces the consumer to fall back to plain text processing for the whole block
    Fallback,
}

impl InspectOutput {
    pub fn map_to_take<F>(self, mapFn: F) -> TakeOutput
    where
        F: FnOnce() -> TakeOutput,
    {
        match self {
            InspectOutput::Accept => mapFn(),
            InspectOutput::Last => TakeOutput::Last,
            InspectOutput::Reject => panic!("trying to take rejected line"),
            InspectOutput::Fallback => TakeOutput::Fallback,
        }
    }
}

#[derive(Debug)]
pub enum ParserOutput {
    Pending,
    Finished,
}

pub trait BlockParserInner: fmt::Debug {
    /// Returns information whether or not this block supports a given line
    fn inspect_line(&self, line_type: LineType, line: &str) -> InspectOutput;
    /// Takes a line updating the internal state
    fn take_line(&mut self, line_type: LineType, line: &str) -> TakeOutput;
    /// Processes the accumulated lines
    fn process(&self, lines: Vec<(LineType, String)>) -> Block;
    /// Returns whether or not the parser could
    /// produce a block in its current state
    fn is_valid(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct BlockParser {
    inner: Box<BlockParserInner>,
    lines: Vec<(LineType, String)>,
}

impl BlockParser {
    pub fn process(self) -> Block {
        let Self { mut inner, lines } = self;

        // if the current state is not valid
        // we fall back to a normal paragaraph
        if !inner.is_valid() {
            inner = box ParagraphParser::new();
        }

        inner.process(lines)
    }

    pub fn next_line(&mut self, line_type: LineType, line: String) -> ParserOutput {
        match self.inner.inspect_line(line_type, &line) {
            InspectOutput::Accept => {
                self.lines.push((line_type, line));
                ParserOutput::Pending
            }
            InspectOutput::Last => {
                self.lines.push((line_type, line));
                ParserOutput::Finished
            }
            InspectOutput::Reject => ParserOutput::Finished,
            InspectOutput::Fallback => {
                self.inner = box ParagraphParser::new();

                for ref mut entry in &mut self.lines {
                    entry.0 = LineType::Text;
                }

                ParserOutput::Pending
            }
        }
    }

    pub fn for_line_type(line_type: LineType) -> Option<Self> {
        let inner: Option<Box<BlockParserInner>> = match line_type {
            LineType::Blank | LineType::Heading1 | LineType::Heading2 | LineType::Heading3 => None,
            LineType::Text => Some(box ParagraphParser::new()),
            LineType::Quote => Some(box QuoteParser::new()),
            LineType::Decorator | LineType::Divider => Some(box PreParser::new()),
            _ => unimplemented!(),
        };

        inner.map(|inner| {
            BlockParser {
                inner,
                lines: Vec::new(),
            }
        })
    }
}

impl TextAccumulator {
    pub fn new() -> Self {
        TextAccumulator {
            buffer: String::new(),
        }
    }

    ///
    /// Adds a new line to the current accumulated text.
    ///
    pub fn add(&mut self, line: String) {
        if !self.buffer.is_empty() {
            self.buffer.push_str(" ");
        }

        self.buffer.push_str(line.trim());
    }

    pub fn consume(self) -> Text {
        vec![Inline::Chunk(self.buffer)]
    }
}
